use falp;
use instance;
use std::collections::{HashMap, HashSet};

const MAX_TIMES: u32 = 5;

pub fn lsa(
    instance: &instance::ParsedInstance,
    mut s_star: HashSet<instance::InstanceFace>,
    config: &falp::Config,
) -> HashSet<instance::InstanceFace> {
    for _ in 0..MAX_TIMES {
        for index in 0..instance.get_num_points() {
            let mut old: Option<instance::InstanceFace> = None;
            let mut candidates: HashMap<instance::InstanceFace, u32> = HashMap::new();
            for face in 0..instance.get_num_candidates() {
                let instance_face = instance::InstanceFace::new(index, face);
                if s_star.contains(&instance_face) {
                    old = Some(instance_face);
                }
                let gk = {
                    let mut result = 0;

                    for col in instance.get_collisions(&instance_face).unwrap() {
                        if s_star.contains(col) {
                            if col.index != index {
                                result += 1;
                            }
                        }
                    }

                    result
                };
                candidates.insert(instance_face, gk);
            }

            s_star.remove(&old.unwrap());
            s_star.insert(falp::rclv::rclv(&candidates, config));
        }
    }

    return s_star;
}
