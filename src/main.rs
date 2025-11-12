use crate::utils::solution::Solution;

mod utils;
mod solutions;

use solutions::registry::solutions;

fn main() {
    for sol in solutions() {
        print_sol(sol);
    }
}

fn print_sol(sol: Solution) {
    print!("{} Day {} Part {}: ", sol.year, sol.day, sol.part);
    match sol.calculate() {
        Ok(ok_str) => println!("{}", ok_str),
        Err(err) => println!("ERROR: {}", err.msg)
    }
}
