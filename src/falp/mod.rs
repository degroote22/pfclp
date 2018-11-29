// Step 0. Create the conflict graph (done off-line).
// Step 1. Apply  maximum  nonconflict  labeling  algorithm  to  get  the  set  S1  (label positions without conflict).
// Step 2.Take the set S2 to be all points not contained in S1. For each point in S2, choose a label position with minimum conflict.
// Step 3.Take  the  solution  S  to  be  S1∪  S2.  Calculate  the  value  of  the  objective function f  for  S.
// If  there  are  no  conflicts, exit.  Otherwise,  make  S*  =  S and repeat the steps below t times:
// •Apply  local  search  to  all  points  in  S*  to  produce  a  new  potential   solution S*.
// •Calculate the value of f for S*. If f (S*) < f (S), take S = S*.
mod lsa;
mod mnla;
mod plwc;
mod rclc;
mod rclv;
use calc;
use instance;
const GRASP_MAX_TIMES: u32 = 10;
const ALPHA: f64 = 0.01;

pub enum RclMode {
    Value,
    Cardinality,
}

pub struct Config {
    pub rcl_mode: RclMode,
    pub alpha: Alpha,
}

pub struct Alpha {
    v: f64,
}

impl Alpha {
    pub fn new(v: f64) -> Alpha {
        assert!(v >= 0.0);
        assert!(v <= 1.0);
        Alpha { v }
    }
    fn get(&self) -> f64 {
        self.v
    }
}

pub fn grasp(instance: &instance::ParsedInstance) -> Vec<u8> {
    let mut best = None;
    for _ in 0..GRASP_MAX_TIMES {
        let solution = run(
            &instance,
            &Config {
                alpha: Alpha::new(ALPHA),
                rcl_mode: RclMode::Value,
            },
        );
        let result = calc::calc(&instance, &solution);
        match best {
            None => {
                best = Some((solution, result));
            }
            Some((_, res)) => {
                if result < res {
                    best = Some((solution, result));
                }
            }
        }
    }

    best.unwrap().0
}

pub fn run(instance: &instance::ParsedInstance, config: &Config) -> Vec<u8> {
    let step1 = mnla::mnla(&instance, config);

    let step2 = plwc::plwc(&instance, step1, config);

    let step3 = lsa::lsa(&instance, step2);

    let mut v: Vec<instance::InstanceFace> = step3.into_iter().collect();

    v.sort_by_key(|k| k.index);

    let mut result: Vec<u8> = vec![];
    for instance_face in v {
        result.push(instance_face.face)
    }

    result
}
