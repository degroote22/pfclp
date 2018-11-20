use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct InstanceFace {
    index: u32,
    face: u8,
}

impl InstanceFace {
    pub fn new(index: u32, face: u8) -> InstanceFace {
        InstanceFace { index, face }
    }

    // pub fn get_index(&self) -> u32 {
    //     self.index
    // }

    // pub fn get_face(&self) -> u8 {
    //     self.face
    // }
}

pub type CollisionVec = Vec<Vec<Vec<InstanceFace>>>;
type CollisionMap = HashMap<InstanceFace, HashSet<InstanceFace>>;

#[derive(Debug)]
pub struct ParsedInstance {
    num_points: u32,
    num_candidates: u8,
    collisions_map: CollisionMap,
}

impl ParsedInstance {
    pub fn new(num_points: u32, num_candidates: u8, collisions: CollisionVec) -> ParsedInstance {
        ParsedInstance {
            num_points,
            num_candidates,
            collisions_map: make_collisions_map(&collisions),
        }
    }

    pub fn get_num_points(&self) -> u32 {
        self.num_points
    }

    pub fn get_num_candidates(&self) -> u8 {
        self.num_candidates
    }

    pub fn get_collisions(&self, with: &InstanceFace) -> Option<&HashSet<InstanceFace>> {
        self.collisions_map.get(with)
    }

    pub fn collides(&self, lhs: &InstanceFace, rhs: &InstanceFace) -> bool {
        match self.get_collisions(lhs) {
            Some(lhs_collisions) => lhs_collisions.contains(rhs),
            None => false,
        }
    }
}

fn make_collisions_map(collisions: &CollisionVec) -> CollisionMap {
    let mut collisions_bag = HashMap::new();

    for (index_point, vector_of_candidates) in collisions.iter().enumerate() {
        for (index_candidate, vector_of_collisions) in vector_of_candidates.iter().enumerate() {
            let face = InstanceFace::new(index_point as u32, index_candidate as u8);
            if !collisions_bag.contains_key(&face) {
                collisions_bag.insert(face, HashSet::new());
            }
            let mut set = collisions_bag.get_mut(&face).unwrap();
            for collision in vector_of_collisions {
                set.insert(collision.clone());
            }
        }
    }

    collisions_bag
}

#[cfg(test)]
mod test_instance {
    use super::*;
    use test_helpers;

    #[test]
    fn test_make_collisions_map() {
        let face_a_0 = InstanceFace { index: 0, face: 0 };
        let face_a_1 = InstanceFace { index: 0, face: 1 };

        let face_b_0 = InstanceFace { index: 1, face: 0 };
        let face_b_1 = InstanceFace { index: 1, face: 1 };

        // collisions
        let point_a_face_0 = vec![face_b_0];
        let point_a_face_1 = vec![face_b_1];
        let point_a = vec![point_a_face_0, point_a_face_1];

        let point_b_face_0 = vec![face_a_0];
        let point_b_face_1 = vec![face_a_1];
        let point_b = vec![point_b_face_0, point_b_face_1];

        let col = vec![point_a, point_b];

        let map = make_collisions_map(&col);

        let mut set0 = HashSet::new();
        set0.insert(face_b_0);
        assert_eq!(map.get(&face_a_0).unwrap(), &set0);

        let non_face0 = InstanceFace {
            index: 1111,
            face: 0,
        };

        let non_face1 = InstanceFace {
            index: 0,
            face: 111,
        };
        let non_face2 = InstanceFace {
            index: 1111,
            face: 111,
        };
        assert_eq!(map.get(&non_face0), None);
        assert_eq!(map.get(&non_face1), None);
        assert_eq!(map.get(&non_face2), None);
    }

    #[test]
    fn test_instance_collides() {
        let instance = test_helpers::generate_simple_instance();
        let face_a_0 = InstanceFace { index: 0, face: 0 };
        let face_a_1 = InstanceFace { index: 0, face: 1 };

        let face_b_0 = InstanceFace { index: 1, face: 0 };
        let face_b_1 = InstanceFace { index: 1, face: 1 };

        assert!(instance.collides(&face_a_0, &face_b_0));
        assert!(instance.collides(&face_b_0, &face_a_0));

        assert!(!instance.collides(&face_a_0, &face_a_0));
        assert!(!instance.collides(&face_a_0, &face_a_1));

        assert!(!instance.collides(&face_b_0, &face_a_1));
        assert!(!instance.collides(&face_a_0, &face_b_1));
    }
}
