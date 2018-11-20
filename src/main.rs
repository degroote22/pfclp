mod bee;
mod breed;
mod calc;
mod greedy;
mod instance;
mod io;
// mod local_search;
mod parser;
mod test_helpers;

extern crate rand;
extern crate regex;
use regex::Regex;
#[macro_use]
extern crate lazy_static;

extern crate time;
use time::PreciseTime;

lazy_static! {
    static ref RE: Regex = Regex::new("\\s").unwrap();
}

fn print_and_local_search(
    _start: PreciseTime,
    instance: &instance::ParsedInstance,
    solution: Vec<u8>,
    name: &str,
) {
    // let end = PreciseTime::now();
    // println!(
    //     "{} ms para executar usando {}",
    //     start.to(end).num_milliseconds(),
    //     name
    // );

    let base = calc::calc(&instance, &solution);
    println!("Resultado da colônia usando {}: {}", name, base);

    // let improved = calc::calc(
    //     &instance,
    //     &local_search::two_opt(&instance, solution.to_vec()),
    // );
    // println!(
    //     "Resultado da colônia usando {} + busca local: {}",
    //     name, improved
    // );
}
fn main() {
    let names = vec![
        "instances/d1000/d1000_01.dat",
        "instances/d1000/d1000_02.dat",
        "instances/d1000/d1000_03.dat",
        "instances/d1000/d1000_04.dat",
        "instances/d1000/d1000_05.dat",
        "instances/d1000/d1000_06.dat",
        "instances/d1000/d1000_07.dat",
        "instances/d1000/d1000_08.dat",
        "instances/d1000/d1000_09.dat",
        "instances/d1000/d1000_10.dat",
        "instances/d1000/d1000_11.dat",
        "instances/d1000/d1000_12.dat",
        "instances/d1000/d1000_13.dat",
        "instances/d1000/d1000_14.dat",
        "instances/d1000/d1000_15.dat",
        "instances/d1000/d1000_16.dat",
        "instances/d1000/d1000_17.dat",
        "instances/d1000/d1000_18.dat",
        "instances/d1000/d1000_19.dat",
        "instances/d1000/d1000_20.dat",
        "instances/d1000/d1000_21.dat",
        "instances/d1000/d1000_22.dat",
        "instances/d1000/d1000_23.dat",
        "instances/d1000/d1000_24.dat",
        "instances/d1000/d1000_25.dat",
    ];
    let max = 10;
    for i in 1..max + 1 {
        println!("Execução {} de {}", i, max);
        for name in names.iter() {
            let instance = parser::parse(&io::read_file(name));
            println!("Starting {}", name);
            run_all_breeds(&instance);
            println!("Ending {}", name);
            println!("");
        }
        println!("");
    }
}

fn run_all_breeds(instance: &instance::ParsedInstance) {
    print_and_local_search(
        PreciseTime::now(),
        &instance,
        greedy::generate(&instance),
        "método guloso",
    );
    print_and_local_search(
        PreciseTime::now(),
        &instance,
        bee::run_hive(&instance, breed::BreedStrategy::RandomRespectfulCombination),
        "RandomRespectfulCombination",
    );
    print_and_local_search(
        PreciseTime::now(),
        &instance,
        bee::run_hive(&instance, breed::BreedStrategy::HalfUniformCrossover),
        "HalfUniformCrossover",
    );
    print_and_local_search(
        PreciseTime::now(),
        &instance,
        bee::run_hive(&instance, breed::BreedStrategy::SegmentedCrossover(0.2)),
        "SegmentedCrossover",
    );
    print_and_local_search(
        PreciseTime::now(),
        &instance,
        bee::run_hive(&instance, breed::BreedStrategy::UniformCrossover),
        "UniformCrossover",
    );
    print_and_local_search(
        PreciseTime::now(),
        &instance,
        bee::run_hive(&instance, breed::BreedStrategy::SinglePointCrossover),
        "SinglePointCrossover",
    );
    print_and_local_search(
        PreciseTime::now(),
        &instance,
        bee::run_hive(&instance, breed::BreedStrategy::TwoPointCrossover),
        "TwoPointCrossover",
    );
    for i in 3..8 {
        print_and_local_search(
            PreciseTime::now(),
            &instance,
            bee::run_hive(&instance, breed::BreedStrategy::KPointCrossover(i)),
            &format!("{}-PointCrossover", i),
        );
    }
}
