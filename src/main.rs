use colombini_sat::solver::Solver;

fn main() {
    let solver = Solver::create("TestData/solver0.cnf").unwrap();
    let solution = solver.solve();

    match solution{
        Some(solution) => println!("SAT: ({:?})", solution),
        None => println!("UNSAT")
    }
}
