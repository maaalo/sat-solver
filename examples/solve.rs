extern crate sat_solver;

use crate::sat_solver::*;
use std::env;

fn main() {
    let cnf = match env::args().nth(1) {
        Some(name) => cnf::read_file(name),
        None       => cnf::read_stdin(),
    };

    let mut solver = solver::Solver::new(cnf);
    let ret = solver.dpll(&clause::Set::new(), &clause::Set::new());
    match ret {
        true => println!("SATISFIABLE"),
        false => println!("UNSATISFIABLE"),
    }
}
