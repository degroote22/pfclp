use bee::agent;
use breed;
use calc;
use instance;
use rand::{thread_rng, Rng};
use std::env;

#[derive(Debug)]
pub struct BeeHive<'a> {
    agents: Vec<agent::BeeAgent>,
    instance: &'a instance::ParsedInstance,
    mutations: u64,
    tick: u64,
    breed_strategy: breed::BreedStrategy,
}

impl<'a> BeeHive<'a> {
    pub fn new(
        instance: &instance::ParsedInstance,
        breed_strategy: breed::BreedStrategy,
    ) -> BeeHive {
        let agents = agent::generate_random_agents(&instance);
        let bee = BeeHive {
            agents,
            instance,
            mutations: 0,
            tick: 0,
            breed_strategy,
        };

        bee
    }

    fn send_employees(&mut self) {
        for index in 0..super::HIVE_SIZE {
            self.send_employee(index as usize);
        }
    }

    fn send_employee(&mut self, index: usize) {
        let employee_nectar = self.agents[index].nectar;
        let employee_memory = self.agents[index].memory.clone();

        // TODO: garantir que são diferentes
        let friend_index = thread_rng().gen_range(0, super::HIVE_SIZE - 1);
        let bee_friend_memory = self.agents.iter().nth(friend_index).unwrap().memory.clone();

        let mutated = breed::breed(
            employee_memory,
            &bee_friend_memory,
            &self.breed_strategy,
            self.instance.get_num_candidates(),
        );

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
        for _ in 0..super::SEND_SCOUTS_NUM {
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

        if self.agents[index as usize].counter > super::MAX_TRIALS {
            let solution = agent::generate_random_solution(
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

    fn get_best_nectar(&mut self) -> u64 {
        self.sort_by_best();

        let best = self.agents[0].nectar;
        best
    }

    fn print(&mut self) {
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
        println!("mutations {:?}", self.mutations);
    }

    fn sort_by_best(&mut self) {
        self.agents.sort_by_key(|k| k.nectar);
    }

    fn send_onlookers(&mut self) {
        // pega algumas melhores e substitui pelas piores
        self.sort_by_best();

        let max = super::HIVE_SIZE - 1;
        for index in 0..super::TO_CLONE_LENGTH {
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

    fn run_loop(&mut self) {
        self.send_employees();

        self.send_onlookers();

        self.send_scouts();
        if self.tick % 60 == 0 {
            self.print();
        }
        self.tick += 1;
    }

    pub fn run_all(&mut self) {
        for _ in 0..super::MAX_ITERATIONS {
            self.run_loop();
        }
    }

    pub fn get_best_solution(&mut self) -> Vec<u8> {
        self.sort_by_best();
        let best = self.agents[0].memory.clone();
        best
    }
}
