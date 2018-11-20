use breed;
use calc;
use instance;
use std::env;

use rand::{thread_rng, Rng};
#[derive(Debug)]
pub struct BeeAgent {
    memory: Vec<u8>,
    nectar: u64,
    counter: u64,
}
const MAX_ITERATIONS: usize = 200;

// essas se mudar uma coisa a outra fica alta demais
// esse balanço foi difícil de achar, portanto não mudar
const HIVE_SIZE: usize = 25;
const MAX_TRIALS: u64 = HIVE_SIZE as u64 / 4;
const SEND_SCOUTS_NUM: usize = HIVE_SIZE / 6;
const TO_CLONE_LENGTH: usize = HIVE_SIZE / 33;

fn get_bounds(v: &Vec<u64>) -> (u64, u64) {
    let mut min: Option<u64> = None;
    let mut max: Option<u64> = None;
    for n in v {
        match max {
            None => {
                max = Some(*n);
            }
            Some(old) => {
                if *n > old {
                    max = Some(*n);
                }
            }
        }

        match min {
            None => {
                min = Some(*n);
            }
            Some(old) => {
                if *n < old {
                    min = Some(*n);
                }
            }
        }
    }
    (min.unwrap(), max.unwrap())
}

fn generate_random_solution(num_candidates: u8, num_points: u32) -> Vec<u8> {
    let mut v = vec![];

    for _ in 0..num_points {
        let num: u8 = thread_rng().gen_range(0, num_candidates);

        v.push(num);
    }

    v
}

fn generate_random_agents(instance: &instance::ParsedInstance) -> Vec<BeeAgent> {
    let mut agents: Vec<BeeAgent> = vec![];

    for _ in 0..HIVE_SIZE {
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

#[derive(Debug)]
pub struct BeeHive<'a> {
    agents: Vec<BeeAgent>,
    instance: &'a instance::ParsedInstance,
    mutations: u64,
    probabilities: Vec<f64>,
    tick: u64,
    breed_strategy: breed::BreedStrategy,
}

impl<'a> BeeHive<'a> {
    pub fn new(
        instance: &instance::ParsedInstance,
        breed_strategy: breed::BreedStrategy,
    ) -> BeeHive {
        let agents = generate_random_agents(&instance);
        let mut bee = BeeHive {
            agents,
            instance,
            mutations: 0,
            tick: 0,
            probabilities: vec![],
            breed_strategy,
        };

        bee.compute_probabilities();

        bee
    }

    fn fix_nectar(nectar: u64, min_nectar: u64) -> f64 {
        // fator de correção que dá resultados bons
        // porque queremos reduzir o nectar
        1.0 / (2.0 + nectar as f64 - min_nectar as f64)
    }

    fn compute_probabilities(&mut self) {
        let nectar_arr: Vec<u64> = self.agents.iter().map(|i| i.nectar).collect();
        let (min_nectar, _max_nectar) = get_bounds(&nectar_arr);
        let sum: f64 = nectar_arr
            .iter()
            .map(|nectar| BeeHive::fix_nectar(*nectar, min_nectar))
            .sum();

        let mut pre_probabilities: Vec<f64> = vec![];

        for nectar in nectar_arr {
            let fit = BeeHive::fix_nectar(nectar, min_nectar);
            pre_probabilities.push(fit as f64 / sum as f64);
        }

        let mut probabilities = vec![pre_probabilities[0]];
        for (index, prob) in pre_probabilities.iter().enumerate().skip(1) {
            let val = probabilities[index - 1] + prob;
            probabilities.push(val);
        }

        self.probabilities = probabilities;
    }

    fn send_employees(&mut self) {
        for index in 0..HIVE_SIZE {
            self.send_employee(index as usize);
        }
    }

    fn select(&self, beta: f64) -> usize {
        for (index, prob) in self.probabilities.iter().enumerate() {
            if beta < *prob {
                return index;
            }
        }

        panic!("O select deveria mandar alguém de acordo com uma probabilidade.");
    }

    fn send_onlookers(&mut self) {
        let mut beta: f64 = thread_rng().gen_range(0.0, 1.0);
        for _ in 0..HIVE_SIZE {
            beta += thread_rng().gen_range(0.0, 1.0);
            beta = beta % 1.0;
            let index = self.select(beta);
            self.send_employee(index);
        }
    }

    fn send_employee(&mut self, index: usize) {
        let employee_nectar = self.agents[index].nectar;
        let employee_memory = self.agents[index].memory.clone();

        // TODO: garantir que são diferentes
        let friend_index = thread_rng().gen_range(0, HIVE_SIZE - 1);
        let bee_friend_memory = self.agents.iter().nth(friend_index).unwrap().memory.clone();

        let mutated = breed::breed(employee_memory, &bee_friend_memory, &self.breed_strategy);

        let mutation_nectar = calc::calc(&self.instance, &mutated);

        if mutation_nectar < employee_nectar {
            self.mutations += 1;
            self.agents[index].reset_counter();
            self.agents[index].set_memory(&mutated);
            self.agents[index].set_nectar(mutation_nectar);
        } else {
            self.agents[index].increase_counter();
        }
    }

    fn send_scouts(&mut self) {
        for _ in 0..SEND_SCOUTS_NUM {
            self.send_scout();
        }
    }

    fn send_scout(&mut self) {
        let mut oldest = None;

        for (index, bee) in self.agents.iter().enumerate() {
            match oldest {
                None => {
                    oldest = Some((index, bee.counter));
                }
                Some((_old_index, old_counter)) => {
                    if old_counter < bee.counter {
                        oldest = Some((index, bee.counter));
                    }
                }
            }
        }

        let (index, _) = oldest.unwrap();

        if self.agents[index as usize].counter > MAX_TRIALS {
            let solution = generate_random_solution(
                self.instance.get_num_candidates(),
                self.instance.get_num_points(),
            );
            let nectar = calc::calc(&self.instance, &solution);
            self.agents[index as usize].set_memory(&solution);
            self.agents[index as usize].reset_counter();
            self.agents[index as usize].set_nectar(nectar);
            self.send_employee(index as usize);
        }
    }

    fn get_mutations(&self) -> u64 {
        self.mutations
    }

    fn get_best_nectar(&self) -> u64 {
        let mut best = self.agents[0].nectar;
        for agent in self.agents.iter().skip(1) {
            let nectar = agent.nectar;
            if nectar < best {
                best = nectar;
            }
        }
        best
    }

    fn print(&self) {
        let args: Vec<String> = env::args().collect();
        let mut debug = false;
        for arg in args {
            if arg == "dbg" {
                debug = true;
            }
        }
        if !debug {
            return;
        }
        println!(
            "mean {}",
            self.agents.iter().map(|x| x.nectar).sum::<u64>() / self.agents.len() as u64
        );
        println!("best nectar {:?}", self.get_best_nectar());
        println!("mutations {:?}", self.get_mutations());
    }

    fn sort_by_best(&mut self) {
        self.agents.sort_by_key(|k| k.nectar);
    }

    fn clone_best(&mut self) {
        self.sort_by_best();

        let max = HIVE_SIZE - 1;
        for index in 0..TO_CLONE_LENGTH {
            if index == 1 {
                continue;
            }
            let old_memory = self.agents[index].memory.clone();
            let nectar = self.agents[index].nectar;

            self.agents[max - index].set_memory(&old_memory);
            self.agents[max - index].set_nectar(nectar);
            self.agents[max - index].reset_counter();
        }
    }

    pub fn run_all(&mut self) {
        for _ in 0..MAX_ITERATIONS {
            self.run_loop();
        }
    }

    fn run_loop(&mut self) {
        self.send_employees();
        self.send_onlookers();

        self.clone_best();

        self.send_scouts();
        if self.tick % 60 == 0 {
            self.print();
        }
        self.tick += 1;

        self.compute_probabilities();
    }

    pub fn get_best_solution(&mut self) -> Vec<u8> {
        self.sort_by_best();
        let best = self.agents[0].memory.clone();
        best
    }
}

pub fn bee_hive(
    instance: &instance::ParsedInstance,
    breed_strategy: breed::BreedStrategy,
) -> Vec<u8> {
    let mut bh = BeeHive::new(&instance, breed_strategy);
    bh.run_all();

    let best = bh.get_best_solution();

    best
}
