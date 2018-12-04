// mod bee;
// mod breed;
mod calc;
// mod greedy;
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

fn make() -> Vec<(instance::ParsedInstance, String)> {
    let mut v = vec![];
    let names = vec![
        "instances/taillard/chH02L24p4.dat",
        "instances/taillard/chH02L32p4.dat",
        "instances/taillard/chH03L16p4.dat",
        "instances/taillard/chH02L42p4.dat",
        "instances/taillard/chH02L48p4.dat",
        "instances/taillard/chH03L24p4.dat",
        "instances/taillard/chH04L16p4.dat",
        "instances/taillard/chH03L28p4.dat",
        "instances/taillard/chH04L18p4.dat",
        "instances/taillard/chH03L32p4.dat",
        "instances/taillard/chH04L21p4.dat",
        "instances/taillard/chH04L24p4.dat",
    ];
    for name in names.iter() {
        let mut instance = parser::parse(&io::read_file(name), Some(505));
        v.push((instance, format!("{} - {}", name, 505)))
    }
    for name in names.iter() {
        let mut instance = parser::parse(&io::read_file(name), Some(5046));
        v.push((instance, format!("{} - {}", name, 5046)))
    }
    for name in names.iter() {
        let mut instance = parser::parse(&io::read_file(name), None);
        v.push((instance, format!("{} - {}", name, "13k")))
    }
    v
}
lazy_static! {
    static ref RE: Regex = Regex::new("\\s").unwrap();
    static ref instances: Vec<(instance::ParsedInstance, String)> = make();
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
}

fn main() {
    for (instance, name) in instances.iter() {
        println!("Starting {} ", name);
        run_all_breeds(&instance);
        println!("");
    }
}

fn run_all_breeds(instance: &'static instance::ParsedInstance) {
    print_and_local_search(
        PreciseTime::now(),
        &instance,
        falp::grasp(&instance),
        "grasp",
    );

    // print_and_local_search(
    //     PreciseTime::now(),
    //     &instance,
    //     bee::run_hive(&instance, breed::BreedStrategy::UniformCrossover),
    //     "Bee Colony + UniformCrossover",
    // );

    // print_and_local_search(
    //     PreciseTime::now(),
    //     &instance,
    //     greedy::generate(&instance),
    //     "método guloso",
    // );
}
