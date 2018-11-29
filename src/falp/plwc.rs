use falp;
use instance;
use std::collections::{HashMap, HashSet};

pub fn plwc(
    instance: &instance::ParsedInstance,
    mut s1: HashSet<instance::InstanceFace>,
    config: &falp::Config,
) -> HashSet<instance::InstanceFace> {
    let mut s2: HashSet<u32> = HashSet::new();

    for i in 0..instance.get_num_points() {
        s2.insert(i);
    }

    for c in s1.iter() {
        let ok = s2.remove(&c.index);
        assert!(ok);
    }

    let mut l_star: HashMap<instance::InstanceFace, u32> = HashMap::new();
    for point in s2.iter() {
        for face in 0..instance.get_num_candidates() {
            let instance_face = instance::InstanceFace::new(*point, face);
            let gk = {
                let mut result = 0;

                for col in instance.get_collisions(&instance_face).unwrap() {
                    if s1.contains(col) {
                        //
                        assert!(col.index != *point);
                        // {
                        result += 1;
                        // }
                    }
                }

                result
            };

            l_star.insert(instance_face, gk);
        }
    }
    loop {
        // Step 1.If S2 is empty, exit with S*as the solution. Otherwise, continue.

        if l_star.len() == 0 {
            assert!(s1.len() as u32 == instance.get_num_points());
            return s1;
        }

        // Step 2.For each point pi. in S2, do step 3.
        // Step 3.For each label position lk of the point pi, do steps 4 and 5.
        // Step 4.Calculate gk, the number of all active labels that overlap with lk.
        // Step 5.Take the label with the smallest gk to be the best label position for point pi. Update S* with this pair (point, label position).
        let instance_face = falp::rclv::rclv(&l_star, config);
        // remove as 4 faces do l_star
        for face in 0..instance.get_num_candidates() {
            let instance_face2 = instance::InstanceFace::new(instance_face.index, face);
            l_star.remove(&instance_face2);
        }

        for col in instance.get_collisions(&instance_face).unwrap() {
            match l_star.get_mut(&col) {
                None => {}
                Some(x) => *x += 1,
            }
        }
        s1.insert(instance_face);
    }
}
// }
