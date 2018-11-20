use instance;

fn faces_from_solution_vec(solution_pre: &Vec<Option<u8>>) -> Vec<Option<instance::InstanceFace>> {
    let mut v = vec![];

    for (index, solution_face) in solution_pre.into_iter().enumerate() {
        match solution_face {
            Some(f) => {
                let instance_face = instance::InstanceFace::new(index as u32, *f);
                v.push(Some(instance_face));
            }
            None => v.push(None),
        }
    }

    v
}

fn can_use_face(
    face: &instance::InstanceFace,
    solution_pre: &Vec<Option<u8>>,
    instance: &instance::ParsedInstance,
) -> bool {
    let mut can = true;

    for solution_face in faces_from_solution_vec(&solution_pre) {
        match solution_face {
            Some(f) => {
                if instance.collides(&face, &f) {
                    can = false;
                }
            }
            None => {}
        }
    }
    can
}

fn generate_pre_greedy_solution(instance: &instance::ParsedInstance) -> Vec<Option<u8>> {
    let mut solution_pre: Vec<Option<u8>> = vec![];

    for _ in 0..instance.get_num_points() {
        solution_pre.push(None);
    }

    'outer: for index in 0..instance.get_num_points() {
        'inner: for face in 0..instance.get_num_candidates() {
            let instance_face = instance::InstanceFace::new(index, face);
            if can_use_face(&instance_face, &solution_pre, &instance) {
                // println!("index {} can use face {}", index, face);
                solution_pre[index as usize] = Some(face);
                continue 'outer;
            }
        }
        solution_pre[index as usize] = Some(0);
    }

    solution_pre
}

pub fn generate(instance: &instance::ParsedInstance) -> Vec<u8> {
    let solution_pre = generate_pre_greedy_solution(&instance);

    let solution: Vec<u8> = solution_pre.iter().map(|s| s.unwrap()).collect();

    solution
}
