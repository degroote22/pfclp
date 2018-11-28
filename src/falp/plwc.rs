use instance;
use std::collections::HashSet;

pub fn plwc(
    instance: &instance::ParsedInstance,
    mut s1: HashSet<instance::InstanceFace>,
) -> HashSet<instance::InstanceFace> {
    let mut s2: HashSet<u32> = HashSet::new();

    for i in 0..instance.get_num_points() {
        s2.insert(i);
    }

    for c in s1.iter() {
        let ok = s2.remove(&c.index);
        assert!(ok);
    }

    loop {
        // Step 1.If S2 is empty, exit with S*as the solution. Otherwise, continue.

        if s2.len() == 0 {
            assert!(s1.len() as u32 == instance.get_num_points());
            return s1;
        }

        // Step 2.For each point pi. in S2, do step 3.
        let mut sorted: Vec<u32> = s2.clone().into_iter().collect();
        sorted.sort();
        for point in sorted {
            // Step 3.For each label position lk of the point pi, do steps 4 and 5.
            let mut best: Option<(instance::InstanceFace, u32)> = None;
            for face in 0..instance.get_num_candidates() {
                // Step 4.Calculate gk, the number of all active labels that overlap with lk.
                let instance_face = instance::InstanceFace::new(point, face);
                let gk = {
                    let mut result = 0;

                    for col in instance.get_collisions(&instance_face).unwrap() {
                        if s1.contains(col) {
                            if col.index != point {
                                result += 1;
                            }
                        }
                    }

                    result
                };

                // update the best
                match best {
                    None => best = Some((instance_face, gk)),
                    Some((old_face, old)) => {
                        if gk < old {
                            best = Some((instance_face, gk))
                        } else if gk == old {
                            assert!(old_face.index == instance_face.index);
                            if old_face.face < instance_face.face {
                                best = Some((instance_face, gk))
                            }
                        }
                    }
                }
            }
            // Step 5.Take the label with the smallest gk to be the best label position for point pi. Update S* with this pair (point, label position).
            let (instance_face, _gk) = best.unwrap();
            s1.insert(instance_face);
            s2.remove(&point);
        }
    }
}
