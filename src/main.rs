mod bee;
mod breed;
mod calc;
mod greedy;
mod instance;
mod io;
mod local_search;
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
    start: PreciseTime,
    instance: &instance::ParsedInstance,
    solution: Vec<u8>,
    name: &str,
) {
    let end = PreciseTime::now();
    println!(
        "{} ms para executar usando {}",
        start.to(end).num_milliseconds(),
        name
    );

    let base = calc::calc(&instance, &solution);
    println!("Resultado da colônia usando {}: {}", name, base);

    let improved = calc::calc(
        &instance,
        &local_search::two_opt(&instance, solution.to_vec()),
    );
    println!(
        "Resultado da colônia usando {} + busca local: {}",
        name, improved
    );
}

fn main() {
    // let instance = parser::parse(&io::read_file("instances/taillard/chH02L24p4.dat"));
    // let instance = parser::parse(&io::read_file("instances/d25/d25_01.dat"));
    let instance = parser::parse(&io::read_file("instances/d1000/d1000_01.dat"));

    print_and_local_search(
        PreciseTime::now(),
        &instance,
        greedy::generate(&instance),
        "método guloso",
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
        bee::run_hive(&instance, breed::BreedStrategy::RandomMess),
        "RandomMess",
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

    for i in 3..5 {
        print_and_local_search(
            PreciseTime::now(),
            &instance,
            bee::run_hive(&instance, breed::BreedStrategy::KPointCrossover(i)),
            &format!("{}-PointCrossover", i),
        );
    }
}
