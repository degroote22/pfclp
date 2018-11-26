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

            for face in 0..instance.get_num_candidates() {
                let instance_face = instance::InstanceFace::new(index, face);

                let gk = {
                    let mut result = 0;

                    for col in instance.get_collisions(&instance_face).unwrap() {
                        if s_star.contains(col) {
                            result += 1;
                        }
                    }

                    result
                };

                // update the best
                match best {
                    None => best = Some((instance_face, gk)),
                    Some((_, old)) => {
                        if gk < old {
                            best = Some((instance_face, gk))
                        }
                    }
                }
            }

            let (instance_face, gk) = best.unwrap();
            if gk != 0 {
                let mut old_instance_face = None;
                for p in s_star.iter() {
                    if p.index == instance_face.index {
                        old_instance_face = Some(*p);
                    }
                }

                s_star.remove(&old_instance_face.unwrap());
                s_star.insert(instance_face);
            }
        }
    }

    return s_star;
}
