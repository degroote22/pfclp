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
use instance;

pub fn run(instance: &instance::ParsedInstance) -> Vec<u8> {
    let step1 = mnla::mnla(&instance);
    let step2 = plwc::plwc(&instance, step1);
    let step3 = lsa::lsa(&instance, step2);

    let mut v: Vec<instance::InstanceFace> = step3.into_iter().collect();

    v.sort_by_key(|k| k.index);

    let mut result: Vec<u8> = vec![];
    for instance_face in v {
        result.push(instance_face.face)
    }

    result
}
