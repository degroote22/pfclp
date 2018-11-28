use instance;
use std::collections::HashSet;
const MAX_TIMES: u32 = 5;

pub fn lsa(
    instance: &instance::ParsedInstance,
    mut s_star: HashSet<instance::InstanceFace>,
) -> HashSet<instance::InstanceFace> {
    for _ in 0..MAX_TIMES {
        for index in 0..instance.get_num_points() {
            let mut best: Option<(instance::InstanceFace, u32)> = None;
            let mut old: Option<instance::InstanceFace> = None;
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

                // update the best
                match best {
                    None => best = Some((instance_face, gk)),
                    Some((old_instance_face, stored_gk)) => {
                        if gk < stored_gk {
                            best = Some((instance_face, gk))
                        } else if gk == stored_gk {
                            // se igual pega o de maior index
                            if old_instance_face.face < instance_face.face {
                                best = Some((instance_face, gk))
                            }
                        }
                    }
                }
            }

            let (instance_face, _gk) = best.unwrap();
            s_star.remove(&old.unwrap());
            s_star.insert(instance_face);
        }
    }

    return s_star;
}
