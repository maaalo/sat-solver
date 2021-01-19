use std::fs::File;
use std::io;
use std::cmp::Ordering::*;
use core::ops::Range;

use crate::{
    clause::Clause,
    clause::Bag,
};

#[derive(Debug, Clone)]
pub struct Cnf {
    pub formula: Vec<Clause>,    // Vector of clause
    pub length: usize,           // Length of formla
    pub mask: Vec<bool>,         // Assign of truth value
    pub count_t: Vec<u16>,       // Count of true
    pub count_f: Vec<u16>,       // Count of false
    var_count: usize,            // Number of variable
    pub history: Vec<u32>,           // History
}

impl Cnf {
    pub fn new(formula: Vec<Clause>, var_count: usize) -> Cnf {
        let len = formula.len();

        let mut count_t = vec![0u16; var_count+1];
        let mut count_f = vec![0u16; var_count+1];
        for clause in &formula {
            for v in &clause.t { count_t[*v] += 1 }
            for v in &clause.f { count_f[*v] += 1 }
        }

        Cnf {
            formula: formula,
            var_count: var_count,
            length: len,
            mask: vec![false; len],
            history: Vec::with_capacity(len),
            count_t: count_t,
            count_f: count_f,
        }
    }

    pub fn var_count(&self) -> usize {
        self.var_count
    }

    pub fn variables(&self) -> Range<usize> {
        1..(self.var_count + 1)
    }
}

fn read_cnf<T: io::BufRead + Sized>(source: T) -> Cnf {
    let mut n = 0;
    let clauses = source.lines().flat_map( |x| {
        let line = x.expect("Error reading a source.");
        let elements: Vec<_> = line.trim().split_whitespace().collect();
        match elements.get(0) {
            None | Some(&"c") => None,
            Some(&"p") => {
                n = elements[2].parse::<usize>().expect("Expected number of variables.");
                None
            },
            _ => {
                let mut t = Bag::new();
                let mut f = Bag::new();
                for v in elements {
                    let n: i32 = v.parse::<i32>().expect("Expected a number.");
                    match n.cmp(&0) {
                        Equal   => break,
                        Greater => t.push(n as usize),
                        Less    => f.push(-n as usize)
                    };
                }
                Some(Clause { t: t, f: f })
            }
        }
    }).collect();
    Cnf::new(clauses, n)
}

pub fn read_file(name: String) -> Cnf {
    let file = io::BufReader::new(File::open(name).expect("File not found."));
    read_cnf(file)
}

pub fn read_stdin() -> Cnf {
    let stdin = io::stdin();
    let locked = stdin.lock();
    read_cnf(locked)
}
