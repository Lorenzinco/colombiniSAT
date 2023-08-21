use colombini_sat::{phi::Phi,solver::solve};

fn main() {
    //solve all 5 test files

        
        let phi = Phi::from_file(&format!("TestData/solver1.cnf")).unwrap();
        println!("Solving 3-SAT with {} literals. Colombini SAT", phi.vars());
        let time = std::time::Instant::now();
        match solve(&phi) {
            Some(assignments) => println!("satisfiable: {:?}", assignments),
            None => println!("unsatisfiable")
        }
        println!("Elapsed: {:?}", time.elapsed());
        /* 
        println!("Solving 3-SAT with {} literals. DPLL", phi.vars());
        let time = std::time::Instant::now();
        match dpll(&phi) {
            Some(assignments) => println!("satisfiable: {:?}", assignments),
            None => println!("unsatisfiable")
        }
        println!("Elapsed: {:?}", time.elapsed());
        */

        
        

        
}
