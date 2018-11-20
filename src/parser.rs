use super::RE;
use instance::*;

enum ParsingResponse {
    NumPoints(u32),
    NumCandidates(u8),
    NumCollisions(u32),
    Collisions(Vec<u32>),
    None,
}

enum ParsingStage {
    NumPoints,
    NumCandidates,
    NumCollisions,
    Collisions,
}

impl ParsingStage {
    pub fn first() -> ParsingStage {
        ParsingStage::NumPoints
    }

    pub fn next(&self) -> ParsingStage {
        match self {
            ParsingStage::NumPoints => ParsingStage::NumCandidates,
            ParsingStage::NumCandidates => ParsingStage::NumCollisions,
            ParsingStage::NumCollisions => ParsingStage::Collisions,
            ParsingStage::Collisions => ParsingStage::NumCollisions,
        }
    }

    pub fn parse(&self, line: &str) -> ParsingResponse {
        let clean_line = RE.replace_all(line, "").to_string();
        if clean_line.len() == 0 {
            return ParsingResponse::None;
        }

        match self {
            ParsingStage::NumPoints => ParsingResponse::NumPoints(
                clean_line
                    .parse::<u32>()
                    .expect("Error while parsing Num Points"),
            ),
            ParsingStage::NumCandidates => ParsingResponse::NumCandidates(
                clean_line
                    .parse::<u8>()
                    .expect("Error while parsing Num Candidates"),
            ),
            ParsingStage::Collisions => {
                let nums = RE
                    .split(line)
                    .map(|n| n.parse::<u32>().expect("Error while parsing collisions"))
                    .collect();
                ParsingResponse::Collisions(nums)
            }
            ParsingStage::NumCollisions => ParsingResponse::NumCollisions(
                clean_line
                    .parse::<u32>()
                    .expect("Error while parsing Num Colisions"),
            ),
        }
    }
}

fn organize_collisions(
    all_collisions: Vec<Vec<u32>>,
    num_candidates: u8,
) -> Vec<Vec<Vec<InstanceFace>>> {
    let collisions = all_collisions.iter().enumerate().fold(vec![], |mut p, c| {
        let (index, items) = c;

        let index_to_place = index / num_candidates as usize;
        let face_to_place = index % num_candidates as usize;
        let mapped_items: Vec<InstanceFace> = items
            .iter()
            // filtra os que são óbvios que estão em conflito pois são do mesmo ponto
            .filter(|&item| {
                let correct_item = *item -1;
                let item_index_to_place = correct_item / num_candidates as u32;
                item_index_to_place != index_to_place as u32
            })
            // coloca num formato de dados melhor, dividindo o item da face
            .map(|item| {
                let correct_item = *item -1;

                let item_index_to_place = correct_item / num_candidates as u32;
                let item_face_to_place = correct_item % num_candidates as u32;

                // println!("item {} index {} face {}",correct_item, item_index_to_place, item_face_to_place);
                return InstanceFace ::new(
                item_index_to_place,
                    item_face_to_place as u8,
            );
            }).collect();

        if face_to_place == 0 {
            // adiciona um vetor com a face 0
            // porque este vetor não existe ainda
            let mut v = vec![];
            v.push(mapped_items);
            p.push(v);
            return p;
        } else {
            p[index_to_place].push(mapped_items);
            return p;
        }
    });
    collisions
}

pub fn parse(contents: &str) -> ParsedInstance {
    let r = _parse(contents);

    ParsedInstance::new(r.num_points, r.num_candidates, r.collisions)
}

#[derive(Debug)]
struct ParsedResponse {
    num_points: u32,
    num_candidates: u8,
    collisions: CollisionVec,
}

fn _parse(contents: &str) -> ParsedResponse {
    let lines = contents.lines();
    let mut num_points: Option<u32> = None;
    let mut num_candidates: Option<u8> = None;
    let mut parsing_stage = ParsingStage::first();
    let mut all_collisions = Vec::new();

    for line in lines {
        let response = parsing_stage.parse(line);

        match response {
            ParsingResponse::None => {}
            ParsingResponse::NumCandidates(num) => {
                num_candidates = Some(num);
                parsing_stage = parsing_stage.next();
            }
            ParsingResponse::NumPoints(num) => {
                num_points = Some(num);
                parsing_stage = parsing_stage.next();
            }
            ParsingResponse::NumCollisions(_n) => {
                parsing_stage = parsing_stage.next();
            }
            ParsingResponse::Collisions(vec) => {
                all_collisions.push(vec);
                parsing_stage = parsing_stage.next();
            }
        }
    }

    ParsedResponse {
        num_points: num_points.unwrap(),
        num_candidates: num_candidates.unwrap(),
        collisions: organize_collisions(all_collisions, num_candidates.unwrap()),
    }
}

#[cfg(test)]
mod test_parser {
    use super::*;
    use io;
    use parser;

    #[test]
    fn test_parser() {
        let file_contents = io::read_file("instances/d25/d25_01.dat");
        let parsed = parser::_parse(&file_contents);

        assert_eq!(parsed.collisions.len(), parsed.num_points as usize);
        for col in parsed.collisions.iter() {
            assert_eq!(col.len(), parsed.num_candidates as usize);
        }

        let result = &parsed.collisions[0][0];
        let expected: Vec<InstanceFace> = vec![15, 17, 26, 39, 69, 90, 91, 95, 96]
            .iter()
            .map(|item| {
                let correct_item = *item - 1;
                let item_index_to_place = correct_item / parsed.num_candidates as u32;
                let item_face_to_place = correct_item % parsed.num_candidates as u32;
                return InstanceFace::new(item_index_to_place, item_face_to_place as u8);
            }).collect();
        assert_eq!(result, &expected);
    }
}
