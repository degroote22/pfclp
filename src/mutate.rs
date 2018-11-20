use rand::{thread_rng, Rng};

const MAX_SLICES: usize = 8;
const MUTATIONS_PER_SOLUTION_SIZE: u32 = 25;

pub fn try0(mut memory_copy: Vec<u8>, friend: &Vec<u8>, num_points: u32) -> Vec<u8> {
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
