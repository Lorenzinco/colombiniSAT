use colombini_sat::{phi::Phi,solver::unrestrained_vector, clause::Clause};

fn main() {
    //solve all 5 test files

        /* 
        let phi = Phi::from_file(&format!("TestData/uf150-01.cnf")).unwrap();
        println!("Solving 3-SAT with {} literals.", phi.vars());
        let time = std::time::Instant::now();
        match solve(&phi) {
            Some(assignments) => println!("satisfiable: {:?}", assignments),
            None => println!("unsatisfiable")
        }
        println!("Elapsed: {:?}", time.elapsed());
        */
        let mut phi = Phi::from_file(&format!("TestData/uf150-01.cnf")).unwrap();
        
        let clause = Clause::new_c1(1);
        phi.clauses.push(clause);
        let clause = Clause::new_c1(2);
        phi.clauses.push(clause);
        let clause = Clause::new_c1(3);
        phi.clauses.push(clause);
        let clause = Clause::new_c1(4);
        phi.clauses.push(clause);
        let clause = Clause::new_c1(5);
        phi.clauses.push(clause);
        let clause = Clause::new_c1(121);
        phi.clauses.push(clause);
        phi = phi.autoreduce();
        println!("{phi}");
        let unrestrained = unrestrained_vector(&phi);
        for i in 0..phi.vars(){
            if !unrestrained.contains(&i) {
                println!("{} is restrained", i);
            } 
        }
        let mut phi_prime = phi.phi_prime(119);
        println!("{phi_prime}");
        

        
}
