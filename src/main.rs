mod bee;
mod calc;
mod greedy;
mod instance;
mod io;
mod ls;
mod mutate;
mod parser;
mod test_helpers;

extern crate rand;
extern crate regex;
use regex::Regex;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref RE: Regex = Regex::new("\\s").unwrap();
}

fn main() {
    // let instance = parser::parse(&io::read_file("instances/taillard/chH02L24p4.dat"));
    // let instance = parser::parse(&io::read_file("instances/d25/d25_01.dat"));
    let instance = parser::parse(&io::read_file("instances/d1000/d1000_01.dat"));

    let greedy_solution = ls::two_opt(&instance, greedy::generate(&instance));
    let result_greedy = calc::calc(&instance, &greedy_solution);

    let solution_bee: Vec<u8> = bee::bee_hive(&instance);

    let result_bee_pure = calc::calc(&instance, &solution_bee);

    let solution_bee_improved = ls::two_opt(&instance, solution_bee);
    let result_bee_improved = calc::calc(&instance, &solution_bee_improved);

    println!("Resultado guloso: {}", result_greedy);
    println!("Resultado da colônia: {}", result_bee_pure);
    println!("Resultado após busca local: {}", result_bee_improved);
}
