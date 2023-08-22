use colombini_sat::{phi::Phi,solver::solve};
use splr::*;
use std::path::Path;

fn main() {
    //solve all 5 test files

    
    println!("Solving 3-SAT with 175 literals. SPLR SAT");
    let config = Config::from(Path::new("TestData/solver6.cnf"));

    let time = std::time::Instant::now();
    let mut s = Solver::build(&config).unwrap();
    if let Ok(ans) = s.solve() {
        println!("{:?}", ans);
    }
    
    println!("Elapsed: {:?}", time.elapsed());
        
    let phi = Phi::from_file(&format!("TestData/solver6.cnf")).unwrap();
    println!("Solving 3-SAT with {} literals. Colombini SAT", phi.vars());
    let time = std::time::Instant::now();
    match solve(&phi) {
        Some(assignments) => {
            print!("SAT([");
            for (i,assignment) in assignments.iter().enumerate() {
                match assignment {
                    true => print!("{}, ", (i+1)),
                    false => print!("-{}, ", (i+1))
                }
            }
            println!(")]");
        },
        None => println!("UNSAT")
    }
    println!("Elapsed: {:?}", time.elapsed());

        
        

        
}
