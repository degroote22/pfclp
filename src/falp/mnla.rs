use falp;
use instance;
use std::collections::{HashMap, HashSet};

pub fn mnla(
    instance: &instance::ParsedInstance,
    config: &falp::Config,
) -> HashSet<instance::InstanceFace> {
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

        let lmin = falp::rclv::rclv(&l_star, config);

        // Step 4. Remove lmin and all nodes adjacent to it from L*.
        s1.insert(lmin);
        l_star.remove(&lmin);

        // update collisions
        let collisions = instance.get_collisions(&lmin).unwrap();
        for col in collisions {
            // remove e vê se já foi removido antes
            // se não foi removido antes, diminui um grau do item
            if let None = l_star.remove(col) {
                continue;
            }

            // Recalculate the degrees
            let c2 = instance.get_collisions(col).unwrap();
            for c in c2 {
                match l_star.get_mut(c) {
                    Some(x) => {
                        *x -= 1;
                    }
                    None => {}
                }
            }
        }

        // Step 5. Go to step 1.
    }
}
