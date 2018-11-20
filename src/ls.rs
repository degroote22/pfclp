use calc;
use instance;

pub fn two_opt(instance: &instance::ParsedInstance, mut solution: Vec<u8>) -> Vec<u8> {
    let mut old_calc = calc::calc(&instance, &solution);
    println!("starting twopt");
    'inner: for (index, solution_face) in solution.clone().iter().enumerate() {
        for maybe_face in 0..instance.get_num_candidates() {
            if maybe_face != *solution_face {
                solution[index] = maybe_face;
                let new_calc = calc::calc(&instance, &solution);

                if new_calc < old_calc {
                    old_calc = new_calc;
                    continue 'inner;
                    // return solution;
                }
            }
        }
        solution[index] = *solution_face;
    }

    return solution;
}
