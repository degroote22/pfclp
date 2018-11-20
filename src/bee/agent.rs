use calc;
use instance;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct BeeAgent {
    pub memory: Vec<u8>,
    pub nectar: u64,
    pub counter: u64,
}

pub fn generate_random_solution(num_candidates: u8, num_points: u32) -> Vec<u8> {
    let mut v = vec![];

    for _ in 0..num_points {
        let num: u8 = thread_rng().gen_range(0, num_candidates);

        v.push(num);
    }

    v
}

pub fn generate_random_agents(instance: &instance::ParsedInstance) -> Vec<BeeAgent> {
    let mut agents: Vec<BeeAgent> = vec![];

    for _ in 0..super::HIVE_SIZE {
        let memory =
            generate_random_solution(instance.get_num_candidates(), instance.get_num_points());
        let nectar = calc::calc(&instance, &memory);

        agents.push(BeeAgent::new(memory, nectar))
    }
    agents
}

impl BeeAgent {
    pub fn new(memory: Vec<u8>, nectar: u64) -> BeeAgent {
        BeeAgent {
            memory,
            nectar,
            counter: 0,
        }
    }

    pub fn set_memory(&mut self, memory: &Vec<u8>) {
        for (index, item) in memory.iter().enumerate() {
            self.memory[index] = *item;
        }
    }

    pub fn reset_counter(&mut self) {
        self.counter = 0;
    }

    pub fn increase_counter(&mut self) {
        self.counter += 1;
    }

    pub fn set_nectar(&mut self, nectar: u64) {
        self.nectar = nectar;
    }
}
