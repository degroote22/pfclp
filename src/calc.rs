use instance;
use std::collections::HashSet;

pub fn calc(instance: &instance::ParsedInstance, solution: &Vec<u8>) -> u64 {
    let mut acc = 0;

    let mut solution_bag = HashSet::new();
    for (index, face) in solution.iter().enumerate() {
        let instance_face = instance::InstanceFace::new(index as u32, *face);
        solution_bag.insert(instance_face);
    }

    for instance_face in solution_bag.iter() {
        if let Some(this_face_collisions) = instance.get_collisions(instance_face) {
            for face in this_face_collisions {
                if solution_bag.contains(&face) {
                    acc += 1;
                }
            }
        }
    }

    acc
}

#[cfg(test)]
mod test_calc {
    use super::*;
    use test_helpers;

    #[test]
    fn test_calc_0() {
        let instance = test_helpers::generate_simple_instance();
        let solution = vec![0, 0];

        assert_eq!(calc(&instance, &solution), 2);

        let solution = vec![0, 1];
        assert_eq!(calc(&instance, &solution), 0);
    }
}
