use falp;
use instance;
use rand::{thread_rng, Rng};
use std::collections::HashMap;

pub fn rclv(
    l_star: &HashMap<instance::InstanceFace, u32>,
    config: &falp::Config,
) -> instance::InstanceFace {
    let mut min = None;
    let mut max = None;
    for (_, degree) in l_star.iter() {
        match min {
            None => min = Some(degree),
            Some(old_min) => {
                if degree < old_min {
                    min = Some(degree)
                }
            }
        }
        match max {
            None => max = Some(degree),
            Some(old_max) => {
                if degree > old_max {
                    max = Some(degree)
                }
            }
        }
    }

    let treshold =
        min.unwrap() + ((config.alpha.get() * (max.unwrap() - min.unwrap()) as f64) as u32);

    let mut v = vec![];

    for (face, deg) in l_star.iter() {
        if deg <= &treshold {
            v.push(face)
        }
    }

    let max = (v.len()) as usize;
    let nth: usize = {
        match max {
            0 => 0,
            _ => thread_rng().gen_range(0, max),
        }
    };

    **v.iter().nth(nth).unwrap()
}
