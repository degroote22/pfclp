use instance;
use std::collections::{HashMap, HashSet};

pub fn mnla(instance: &instance::ParsedInstance) -> HashSet<instance::InstanceFace> {
    // Step 0. The nonconflict set S1 is empty. The active node set  L* is equal to the full node set N.
    let mut s1: HashSet<instance::InstanceFace> = HashSet::new();

    let mut l_star: HashMap<instance::InstanceFace, u32> = {
        let mut h = HashMap::new();
        for point in instance.get_all_points() {
            h.insert(point, instance.point_degree(&point));
        }
        h
    };

    loop {
        // Step 1. If  the  active  node  set  L*  is  empty,  exit  with  S1 as  the  result.  Otherwise, do steps 2 to 5.
        if l_star.len() == 0 {
            return s1;
        }

        // Step 2. Calculate degrees of all nodes in L*. -- calculated already

        // Step 3. Select lmin, the node of smallest degree on L*.  Place it in the nonconflict set S1.
        let lmin: instance::InstanceFace = {
            let mut lmin_instance_face: Option<instance::InstanceFace> = None;
            let mut lmin_degree = 0;

            for (instance_face, degree) in l_star.iter() {
                match lmin_instance_face {
                    None => {
                        lmin_instance_face = Some(*instance_face);
                        lmin_degree = *degree;
                    }
                    Some(_old) => {
                        if degree < &lmin_degree {
                            lmin_instance_face = Some(*instance_face);
                            lmin_degree = *degree;
                        } else if degree == &lmin_degree {
                            // choose with the biggest index
                            if _old.index < instance_face.index {
                                lmin_instance_face = Some(*instance_face);
                                lmin_degree = *degree;
                            }
                        }
                    }
                }
            }

            lmin_instance_face.unwrap()
        };

        s1.insert(lmin);

        // Step 4. Remove lmin and all nodes adjacent to it from L*.
        let collisions = instance.get_collisions(&lmin);
        for col in collisions.unwrap() {
            l_star.remove(col);
        }
        l_star.remove(&lmin);

        // Recalculate the degrees

        for (p, _deg) in l_star.clone().iter() {
            let collisions = instance.get_collisions(p).unwrap();
            let mut deg = 0;
            for c in collisions {
                if l_star.contains_key(c) {
                    deg += 1;
                }
            }
            l_star.insert(*p, deg);
        }

        // Step 5. Go to step 1.
    }
}
