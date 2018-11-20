use rand::{thread_rng, Rng};

#[derive(Debug)]
pub enum BreedStrategy {
    RandomMess,
    SinglePointCrossover,
    TwoPointCrossover,
    KPointCrossover(u32),
    UniformCrossover,
}

fn random_mess(mut memory_copy: Vec<u8>, friend: &Vec<u8>) -> Vec<u8> {
    let num_points = memory_copy.len() as u32;

    const MAX_SLICES: usize = 8;
    const MUTATIONS_PER_SOLUTION_SIZE: u32 = 25;
    let mut indexes_to_loop = vec![];
    for _ in 0..thread_rng().gen_range(1, MAX_SLICES) {
        let i = thread_rng().gen_range(0, num_points - 1);
        indexes_to_loop.push(i);
    }

    for i in indexes_to_loop {
        for sum in 0..thread_rng().gen_range(1, MUTATIONS_PER_SOLUTION_SIZE) {
            let index = (sum + i) % (num_points - 1);
            memory_copy[index as usize] = friend[index as usize];
        }
    }

    memory_copy
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

pub fn uniform_crossover(mut memory_copy: Vec<u8>, friend: &Vec<u8>) -> Vec<u8> {
    let len = memory_copy.len() - 1;
    for i in 0..len {
        if rand::random() {
            memory_copy[i] = friend[i];
        }
    }

    memory_copy
}

pub fn breed(
    employee_memory: Vec<u8>,
    bee_friend_memory: &Vec<u8>,
    breed_strategy: &BreedStrategy,
) -> Vec<u8> {
    match breed_strategy {
        BreedStrategy::RandomMess => random_mess(employee_memory, &bee_friend_memory),
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
    }
}
