use rand::{thread_rng, Rng};

#[derive(Debug)]
pub enum BreedStrategy {
    SinglePointCrossover,
    TwoPointCrossover,
    KPointCrossover(u32),
    UniformCrossover,
    HalfUniformCrossover,
    SegmentedCrossover(f64),
    RandomRespectfulCombination,
}

fn single_point_crossover(mut memory_copy: Vec<u8>, friend: &Vec<u8>) -> Vec<u8> {
    let len = memory_copy.len() - 1;

    let start = thread_rng().gen_range(0, len);

    for index in start..len {
        memory_copy[index] = friend[index];
    }

    memory_copy
}

fn two_point_crossover(mut memory_copy: Vec<u8>, friend: &Vec<u8>) -> Vec<u8> {
    let len = memory_copy.len() - 1;

    let random1 = thread_rng().gen_range(0, len);
    let random2 = thread_rng().gen_range(0, len);

    let start = random1.min(random2);

    let end = random2.max(random1);

    for index in start..end {
        memory_copy[index] = friend[index];
    }

    memory_copy
}

fn k_point_crossover(mut memory_copy: Vec<u8>, friend: &Vec<u8>, k: u32) -> Vec<u8> {
    let len = memory_copy.len() - 1;

    let mut v = vec![];
    for _ in 0..k {
        let r = thread_rng().gen_range(0, len);
        v.push(r);
    }

    v.sort();

    let mut start = v[0];
    let mut end = v[1];
    let mut pos = 0;

    for index in 0..len {
        if index >= start && index < end {
            // copia
            memory_copy[index] = friend[index];
        } else if index > end {
            // se pode andar
            if pos != k as usize - 2 {
                // anda
                pos += 1;
                start = v[pos];
                end = v[pos + 1];
            }
        }
    }

    memory_copy
}

fn uniform_crossover(mut memory_copy: Vec<u8>, friend: &Vec<u8>) -> Vec<u8> {
    let len = memory_copy.len() - 1;
    for i in 0..len {
        if rand::random() {
            memory_copy[i] = friend[i];
        }
    }

    memory_copy
}

fn half_uniform_crossover(mut memory_copy: Vec<u8>, friend: &Vec<u8>) -> Vec<u8> {
    let len = memory_copy.len() - 1;
    for i in 0..len {
        // se eh diferente tenta trocar
        if memory_copy[i] != friend[i] {
            if rand::random() {
                memory_copy[i] = friend[i];
            }
        }
    }

    memory_copy
}

fn segmented_crossover(mut memory_copy: Vec<u8>, friend: &Vec<u8>, s: f64) -> Vec<u8> {
    let len = memory_copy.len() - 1;

    // let copy_from_me = false;

    // let s = 0.2;
    let mut wait = 0;
    let mut started = false;
    for i in 0..len {
        if wait >= i {
            if started {
                memory_copy[i] = friend[i];
            }
        } else {
            let q = thread_rng().gen_range(0.0, 1.0);
            let j = thread_rng().gen_range(i, len);

            if q < s {
                started = true;
                wait = j;
            }
        }
    }

    memory_copy
}

fn random_respectful_combination(
    mut memory_copy: Vec<u8>,
    friend: &Vec<u8>,
    num_candidates: u8,
) -> Vec<u8> {
    let len = memory_copy.len() - 1;
    for i in 0..len {
        // se eh diferente tenta trocar
        if memory_copy[i] != friend[i] {
            if rand::random() {
                memory_copy[i] = thread_rng().gen_range(0, num_candidates - 1)
            }
        }
    }

    memory_copy
}

pub fn breed(
    employee_memory: Vec<u8>,
    bee_friend_memory: &Vec<u8>,
    breed_strategy: &BreedStrategy,
    num_candidates: u8,
) -> Vec<u8> {
    match breed_strategy {
        BreedStrategy::SinglePointCrossover => {
            single_point_crossover(employee_memory, &bee_friend_memory)
        }
        BreedStrategy::TwoPointCrossover => {
            two_point_crossover(employee_memory, &bee_friend_memory)
        }
        BreedStrategy::KPointCrossover(k) => {
            k_point_crossover(employee_memory, &bee_friend_memory, *k)
        }
        BreedStrategy::UniformCrossover => uniform_crossover(employee_memory, &bee_friend_memory),
        BreedStrategy::HalfUniformCrossover => {
            half_uniform_crossover(employee_memory, &bee_friend_memory)
        }
        BreedStrategy::SegmentedCrossover(s) => {
            segmented_crossover(employee_memory, &bee_friend_memory, *s)
        }
        BreedStrategy::RandomRespectfulCombination => {
            random_respectful_combination(employee_memory, &bee_friend_memory, num_candidates)
        }
    }
}
