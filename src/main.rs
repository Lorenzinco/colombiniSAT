use std::process::exit;

use colombini_sat::{solver::{Solver, add_decision, add_implication}, clause::Literal};
use petgraph::prelude::DiGraph;

fn main() {


    let solver = Solver::create("TestData/solver3.cnf").unwrap();
    
    println!("Solving 3-SAT formula with {} variables and {} clauses", solver.num_variables(), solver.num_clauses());
    let start = std::time::Instant::now();
    let solution = solver.solve();
    match solution{
        Some(solution) => println!("SAT: ({:?})", solution),
        None => println!("UNSAT")
    }
    println!("Elapsed: {:?}", start.elapsed());
}
