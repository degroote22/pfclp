use breed;
use instance;

mod agent;
mod hive;

// essas constantes são acessadas pelos outros módulos da pasta
const MAX_ITERATIONS: usize = 600;

// essas se mudar uma coisa a outra fica alta demais
// esse balanço foi difícil de achar, portanto não mudar
const HIVE_SIZE: usize = 50;
const MAX_TRIALS: u64 = HIVE_SIZE as u64 / 4;
const SEND_SCOUTS_NUM: usize = HIVE_SIZE / 6;
const TO_CLONE_LENGTH: usize = HIVE_SIZE / 33;

pub fn run_hive(
    instance: &instance::ParsedInstance,
    breed_strategy: breed::BreedStrategy,
) -> Vec<u8> {
    let mut bee_hive = hive::BeeHive::new(&instance, breed_strategy);
    bee_hive.run_all();
    bee_hive.get_best_solution()
}
