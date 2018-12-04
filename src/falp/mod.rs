mod lsa;
mod mnla;
mod plwc;
mod rclv;
use calc;
use instance;
use std::sync::mpsc::channel;
use std::thread;

// CUIDADO!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
const GRASP_MAX_TIMES: u32 = 20;
const NUMBER_OF_THREADS: u32 = 5; // LEMBRAR DE USAR DIVISAO EXATA!!!!!!!!
const WORK_PER_THREAD: u32 = GRASP_MAX_TIMES / NUMBER_OF_THREADS; // ISSO NAO PODE TER RESTO!!!!!!!!!!

// o código que divide o trabalho para as threads é simples
// e não convém complicar
// CUIDADO!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

const ALPHA: f64 = 0.01;

pub struct Config {
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

pub fn grasp(instance: &'static instance::ParsedInstance) -> Vec<u8> {
    let mut best = None;

    // usando multi-produtor único-consumidor
    let (tx, rx) = channel();

    // cada thread tem WORK_PER_THREAD número de tentativas
    for _ in 0..NUMBER_OF_THREADS {
        let tx = tx.clone();

        thread::spawn(move || {
            for _ in 0..WORK_PER_THREAD {
                let solution = run(
                    &instance,
                    &Config {
                        alpha: Alpha::new(ALPHA),
                    },
                );
                let objective_function = calc::calc(&instance, &solution);
                tx.send((solution, objective_function)).unwrap();
            }
        });
    }

    // recebe os resultados das threads e salva o melhor
    for _ in 0..GRASP_MAX_TIMES {
        let (solution, objective_function) = rx.recv().unwrap();
        match best {
            None => {
                best = Some((solution, objective_function));
            }
            Some((_, best_objective_function)) => {
                if objective_function < best_objective_function {
                    best = Some((solution, objective_function));
                }
            }
        }
    }

    // retorna somente a solução
    best.unwrap().0
}

pub fn run(instance: &instance::ParsedInstance, config: &Config) -> Vec<u8> {
    // Step 0. Create the conflict graph (done off-line).

    // Step 1. Apply  maximum  nonconflict  labeling  algorithm  to  get  the  set  S1  (label positions without conflict).
    let step1 = mnla::mnla(&instance, config);

    // Step 2.Take the set S2 to be all points not contained in S1. For each point in S2, choose a label position with minimum conflict.
    let step2 = plwc::plwc(&instance, step1, config);

    // Step 3.Take  the  solution  S  to  be  S1∪  S2.  Calculate  the  value  of  the  objective function f  for  S.
    // If  there  are  no  conflicts, exit.  Otherwise,  make  S*  =  S and repeat the steps below t times:
    // •Apply  local  search  to  all  points  in  S*  to  produce  a  new  potential   solution S*.
    // •Calculate the value of f for S*. If f (S*) < f (S), take S = S*.
    let step3 = lsa::lsa(&instance, step2, config);

    // converter para a representação comum desta base de código, que é um vetor
    // para a solução. a posição do vetor é o index do ponto e o valor qual canidato escolhido
    let mut v: Vec<instance::InstanceFace> = step3.into_iter().collect();

    v.sort_by_key(|k| k.index);

    let mut result: Vec<u8> = vec![];
    for instance_face in v {
        result.push(instance_face.face)
    }

    result
}
