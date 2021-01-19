#![feature(test)]

extern crate sat_solver;
extern crate test;

use crate::sat_solver::*;
use test::Bencher;

#[bench]
fn bench_strategy_select(b: &mut Bencher) {
    let cnf = cnf::read_file("./cnf/uf75-01.cnf".to_string());
    let solver = solver::Solver::new(cnf);
    b.iter(|| { solver.strategy_select(&clause::Set::new(), &clause::Set::new()); });
}

#[bench]
fn bench_assign(b: &mut Bencher) {
    let cnf = cnf::read_file("./cnf/uf75-01.cnf".to_string());
    let mut solver = solver::Solver::new(cnf);
    b.iter(|| { solver.assign(19, false); });
}

#[bench]
fn bench_dpll(b: &mut Bencher) {
    b.iter(|| { 
        let cnf = cnf::read_file("./cnf/quinn.cnf".to_string());
        let mut solver = solver::Solver::new(cnf);
        solver.dpll(&clause::Set::new(), &clause::Set::new()); 
    });
}
