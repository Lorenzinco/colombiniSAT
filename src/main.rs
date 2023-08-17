use colombini_sat::{phi::Phi,solver::solve};

fn main() {
    //solve all 5 test files

        let phi = Phi::from_file(&format!("TestData/solver20-0.cnf")).unwrap();
        match solve(&phi) {
            Some(assignments) => println!("satisfiable: {:?}", assignments),
            None => println!("unsatisfiable")
        }
}
