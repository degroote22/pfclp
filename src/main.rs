mod bee;
mod breed;
mod calc;
mod greedy;
mod instance;
mod io;
// mod local_search;
mod falp;
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
    println!("{}:", name);
    println!("{}ms", start.to(end).num_milliseconds(),);

    let result = calc::calc(&instance, &solution);
    let conflicting = result / 2;

    let rls: f64 = 100.0 * (conflicting as f64 / instance.get_num_points() as f64);
    let rls_text = format!("{:.*}", 2, 100.0 - rls);
    println!("RLS: {}%", rls_text);

    // let improved = calc::calc(
    //     &instance,
    //     &local_search::two_opt(&instance, solution.to_vec()),
    // );
    // let conflicting = improved / 2;
    // let rls: f64 = 100.0 * (conflicting as f64 / instance.get_num_points() as f64);
    // let rls_text = format!("{:.*}", 2, 100.0 - rls);
    // println!("RLS% após busca local: {}%", rls_text);
    // println!("");
}
fn main() {
    let names = vec!["instances/d1000/d1000_01.dat"];
    // let max = 10;
    // for i in 1..max + 1 {
    // println!("Execução {} de {}", i, max);
    for name in names.iter() {
        let mut instance = parser::parse(&io::read_file(name));
        // instance.clip_num_points(505);
        println!("Starting {}", name);
        run_all_breeds(&instance);
        println!("");
    }
    println!("");
    // }
}

fn run_all_breeds(instance: &instance::ParsedInstance) {
    print_and_local_search(
        PreciseTime::now(),
        &instance,
        greedy::generate(&instance),
        "método guloso",
    );

    print_and_local_search(PreciseTime::now(), &instance, falp::run(&instance), "falp");

    // print_and_local_search(
    //     PreciseTime::now(),
    //     &instance,
    //     bee::run_hive(&instance, breed::BreedStrategy::RandomRespectfulCombination),
    //     "RandomRespectfulCombination",
    // );
    // print_and_local_search(
    //     PreciseTime::now(),
    //     &instance,
    //     bee::run_hive(&instance, breed::BreedStrategy::HalfUniformCrossover),
    //     "HalfUniformCrossover",
    // );
    // print_and_local_search(
    //     PreciseTime::now(),
    //     &instance,
    //     bee::run_hive(&instance, breed::BreedStrategy::SegmentedCrossover(0.2)),
    //     "SegmentedCrossover",
    // );
    print_and_local_search(
        PreciseTime::now(),
        &instance,
        bee::run_hive(&instance, breed::BreedStrategy::UniformCrossover),
        "Bee Colony + UniformCrossover",
    );
    // print_and_local_search(
    //     PreciseTime::now(),
    //     &instance,
    //     bee::run_hive(&instance, breed::BreedStrategy::SinglePointCrossover),
    //     "SinglePointCrossover",
    // );
    // print_and_local_search(
    //     PreciseTime::now(),
    //     &instance,
    //     bee::run_hive(&instance, breed::BreedStrategy::TwoPointCrossover),
    //     "TwoPointCrossover",
    // );
    // for i in 3..8 {
    //     print_and_local_search(
    //         PreciseTime::now(),
    //         &instance,
    //         bee::run_hive(&instance, breed::BreedStrategy::KPointCrossover(i)),
    //         &format!("{}-PointCrossover", i),
    //     );
    // }
}
