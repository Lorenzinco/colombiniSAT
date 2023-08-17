use std::{collections::HashMap, vec};

use crate::{phi::Phi, error::Error, clause::Clause};
use petgraph::{graph::{Graph, NodeIndex}, algo};

/*
Core idea: at each step identify the i-th literal that is forced to be true or false within their i-th phi_prime
phi_prime_i being a subset of phi, composed by the clauses that contains the literal i.
*/
pub fn solve_2_sat(phi: &Phi) -> Result<(), Error>
{
    let mut graph = Graph::<isize,i8>::new();
    let mut added_literals = HashMap::<isize,NodeIndex>::new();

    //remove unit clauses
    let rphi = phi.autoreduce();
    //check that all clauses in phi are 2-sat and
    //for each clause in phi, create a graph composed by each literal being a node 
    //and each clause impling two edges between the two literals in the clause
    for clause in &rphi.clauses
    {
        match clause
        {
            Clause::C3(_,_,_) => { return Err(Error::new("Not a 2-sat formula").into()) }
            Clause::C2(l1,l2) => {

                for i in 0..2
                {
                    let index1: isize = if i == 0 { -l1.as_isize() } else { l1.as_isize() };
                    let index2 = if i == 0 { l2.as_isize() } else { -l2.as_isize() };

                    let mut n1 = added_literals.get(&index1).copied();
                    match n1 {
                        None => {
                            let node = graph.add_node(index1);
                            n1 = Some(node);
                            added_literals.insert(index1, node);
                        }
                        Some(_) => {}
                    }
                    let mut n2 = added_literals.get(&index2).copied();
                    match n2 {
                        None => {
                            let node = graph.add_node(index2);
                            n2 = Some(node);
                            added_literals.insert(index2, node);
                        }
                        Some(_) => {}
                    }
                    graph.add_edge(n1.unwrap(), n2.unwrap(), 1);
                }
            },
            Clause::C1(_) => { unreachable!() },
            Clause::Empty => { return Err(Error::new("Not satisfiable, empty clause given.").into()) },
        }
    }

    //find all completely connected groups
    let sccs = algo::tarjan_scc(&graph);
    //explore the groups and find if there is a node with a completely connected (two way) path to its negation
    for scc in sccs
    {
        for node in &scc
        {
            let index = graph[*node];
            let negation = -index;
            let negation_node = added_literals.get(&negation).copied();
            if let Some(negation_node) = negation_node
            {
                if scc.contains(&negation_node)
                {
                    return Err(Error::new("Not satisfiable, found a node with a completely connected path to its negation."));
                }
            }
        }
    }
    Ok(())
}

pub fn solve_2_sat_with_assignments(phi: &Phi) -> Result<Vec<Option<bool>>, Error>
{
    let mut graph = Graph::<isize,i8>::new();
    let mut added_literals = HashMap::<isize,NodeIndex>::new();

    //remove unit clauses
    let rphi = phi.autoreduce();
    //check that all clauses in phi are 2-sat and
    //for each clause in phi, create a graph composed by each literal being a node 
    //and each clause impling two edges between the two literals in the clause
    for clause in &rphi.clauses
    {
        match clause
        {
            Clause::C3(_,_,_) => { return Err(Error::new("Not a 2-sat formula").into()) }
            Clause::C2(l1,l2) => {

                for i in 0..2
                {
                    let index1: isize = if i == 0 { -l1.as_isize() } else { l1.as_isize() };
                    let index2 = if i == 0 { l2.as_isize() } else { -l2.as_isize() };

                    let mut n1 = added_literals.get(&index1).copied();
                    match n1 {
                        None => {
                            let node = graph.add_node(index1);
                            n1 = Some(node);
                            added_literals.insert(index1, node);
                        }
                        Some(_) => {}
                    }
                    let mut n2 = added_literals.get(&index2).copied();
                    match n2 {
                        None => {
                            let node = graph.add_node(index2);
                            n2 = Some(node);
                            added_literals.insert(index2, node);
                        }
                        Some(_) => {}
                    }
                    graph.add_edge(n1.unwrap(), n2.unwrap(), 1);
                }
            },
            Clause::C1(_) => { unreachable!() },
            Clause::Empty => { return Err(Error::new("Not satisfiable, empty clause given.").into()) },
        }
    }

    //find all completely connected groups
    let sccs = algo::tarjan_scc(&graph);



    //explore the groups and find if there is a node with a completely connected (two way) path to its negation
    for scc in sccs
    {
        for node in &scc
        {
            let index = graph[*node];
            let negation = -index;
            let negation_node = added_literals.get(&negation).copied();
            if let Some(negation_node) = negation_node
            {
                if scc.contains(&negation_node)
                {
                    return Err(Error::new("Not satisfiable, found a node with a completely connected path to its negation."));
                }
            }
        }
    }

    //solve with assignments
    let assignments = dpll(phi);
    match assignments {
        Some(assignment) => {
            Ok(assignment)
        },
        None => { Err(Error::new("unreachable!")) }
    }
}

fn _dpll(phi: &Phi, mut assignments: Vec<Option<bool>>) -> Option<Vec<Option<bool>>>
{
    let new_phi = phi.autoreduce_with_assignments(&mut assignments);
    
    if new_phi.clauses.len() == 0
    {
        return Some(assignments);
    }
    else if new_phi.clauses[0] == Clause::Empty
    {
        return None;
    }
    else 
    {
        //find a literal that is not assigned
        let literals = new_phi.get_variables();
        let mut found = false;
        let mut literal = 0;
        for l in literals
        {
            if assignments[l] == None
            {
                found = true;
                literal = l;
                break;
            }
        }
        if !found
        {
            let final_phi = new_phi.reduce(&assignments);
            if final_phi.clauses.len() == 0
            {
                return Some(assignments);
            }
            else if final_phi.clauses[0] == Clause::Empty
            {
                return None;
            }
            else
            {
                panic!("This should not happen")
            }
        }
        //try to assign it to true
        assignments[literal] = Some(true);
        if let Some(assignments_true) = _dpll(&new_phi, assignments.clone())
        {
            return Some(assignments_true);
        }
        //try to assign it to false
        assignments[literal] = Some(false);
        if let Some(assignments_false) = _dpll(&new_phi, assignments.clone())
        {
            return Some(assignments_false);
        }
        //if both fail, reset the assignment and return false
        assignments[literal] = None;
        return None;
    }
}

pub fn dpll(phi: &Phi) -> Option<Vec<Option<bool>>>
{
    let assignments: Vec<Option<bool>> = vec![None;phi.vars()];
    if let Some(assignments) = _dpll(phi, assignments)
    {
        Some(assignments)
    }
    else
    {
        None
    }
}

pub fn assign_2_sat(phi_prime:&mut Phi, assignments: &mut Vec<Option<bool>> ) -> Result<Vec<Option<bool>>,Error>{
    
    if phi_prime.clauses.len() == 0 {
        return Ok(assignments.clone());
    }
    match phi_prime.clauses[0]{
        Clause::Empty => {return Err(Error::new("Not satisfiable, empty clause given.").into())},
        _ => {}
    }

    let mut phi = phi_prime.clone();
    let mut assignment = assignments.clone();
    phi = phi.autoreduce_with_assignments(&mut assignment);

    if phi.clauses.len() == 0 {
        return Ok(assignment);
    }

    assignments[phi.clauses[0].literals_vector()[0].index] = Some(true);
    match assign_2_sat(phi_prime, assignments){
        Ok(_) => {return Ok(assignments.clone())},
        Err(_) => {
            assignments[phi.clauses[0].literals_vector()[0].index] = Some(false);
            match assign_2_sat(phi_prime, assignments){
                Ok(_) => {return Ok(assignments.clone())},
                Err(_) => {return Err(Error::new("Not satisfiable, empty clause given.").into())}
            }
        }
    }
}

pub fn solve(phi: &Phi) -> Option<Vec<bool>>
{
    let mut phi = phi.clone();
    let n_vars = phi.vars();
    let mut assignment: Vec<Option<bool>> = vec![None;n_vars];
    while phi.clauses.len() > 0{
        phi = phi.autoreduce();
        //check if phi is empty
        if phi.clauses.len() > 0 && phi.clauses[0] == Clause::Empty{
            return None;
        }
        //for each variable, check if it is forced to be true or false
        let literals = phi.get_variables();
        let unrestrained = unrestrained_vector(&phi);
        //create a vector of assignments for each literal in literals
        let mut assignments_for_each_literal: Vec<Vec<Option<bool>>> = vec![vec![None;n_vars]; n_vars];
        let mut last_forced_literal: Option<usize> = None;
        //for every literal in literals that isnt in the reserve list
        for literal in literals{
            if !unrestrained.contains(&literal){
                last_forced_literal = Some(literal);
                //check if it is forced to be true or false, at this point it must be one or another
                let phi_prime = phi.phi_prime(literal);
                assignment[literal] = Some(true);
                let phi_prime_true = phi_prime.reduce(&assignment);
                let result_true = solve_2_sat_with_assignments(&phi_prime_true);
                if result_true.is_ok(){
                    assignments_for_each_literal[literal] = result_true.unwrap();
                    //check if in any other assignment theres at least one literal with opposite value
                }
                assignment[literal] = Some(false);
                let phi_prime_false = phi_prime.reduce(&assignment);
                let result_false = solve_2_sat_with_assignments(&phi_prime_false);
                if result_false.is_ok(){
                    assignments_for_each_literal[literal] = result_false.unwrap();
                }
                println!("Literal {literal} found the assignment {:?}", assignments_for_each_literal[literal]);
                println!("{}",assignments_for_each_literal[literal].len());
                for i in 0..n_vars{
                    if i == literal{
                        continue;
                    }
                    for j in 0..n_vars{
                        match assignments_for_each_literal[i][j]{
                            Some(_)=>{
                                match assignments_for_each_literal[literal][j]{
                                    Some(value)=>{
                                        if value != assignments_for_each_literal[i][j].unwrap(){
                                            return None;
                                        }
                                    }
                                    None=>{}
                                }
                            }
                            None=>{}
                        }
                    }
                }
                //merge assignment and assignments_for_each_literal[literal]
                for i in 0..n_vars{
                    println!("{} {}", i, literal);
                    if assignments_for_each_literal[literal][i].is_some(){
                        assignment[i] = assignments_for_each_literal[literal][i];
                    }
                }
            }
        }

        if last_forced_literal.is_none()
        {
            //if no literal is forced to be true or false, pick one and assign it to true
            let literals = phi.get_variables();
            if literals.len() > 0{
                let literal = literals[0];
                assignment[literal] = Some(false);
                phi = phi.reduce(&assignment);
            }
        }
    }
    //return the assignment vector, if some value is still none fill it with false
    Some(assignment.iter().map(|x| x.unwrap_or(false)).collect())
}

///Returns a list of variables that are not forced to be true or false
pub fn unrestrained_vector(phi: &Phi) -> Vec<usize>
{
    let mut reserves: Vec<usize> = vec![];
    let var_count = phi.vars();
    let vars = phi.get_variables();
    let mut assignment: Vec<Option<bool>> = vec![None;var_count];
    for index in &vars{
        
        let phi_prime = phi.phi_prime(*index);
        assignment[*index] = Some(true);
        let phi_prime_true = phi_prime.reduce(&assignment);
        assignment[*index] = Some(false);
        let phi_prime_false = phi_prime.reduce(&assignment);
        assignment[*index] = None;
        let result_true = solve_2_sat(&phi_prime_true).is_ok();
        let result_false = solve_2_sat(&phi_prime_false).is_ok();
        if result_true && result_false{
            reserves.push(*index);
        } 
        
        /*
        let phi_second = phi.phi_second(*index);
        assignment[*index] = Some(true);
        if solve_2_sat(&phi_second).is_ok(){
            reserves.push(*index);
        } 
        */
    } 
    reserves
}


#[cfg(test)]
mod tests
{
    use crate::phi::*;
    use crate::clause::*;
    #[test]
    fn solve_2_sat()
    {
        let phi = Phi{
            clauses: vec![
                Clause::new_c2(1,-2),
                Clause::new_c2(-1,2),
                Clause::new_c2(-1,-2),
                Clause::new_c2(1,-3),
            ]
        };
        let result = super::solve_2_sat(&phi);
        assert!(result.is_ok());
    }

    #[test]
    fn unrestrained_vector(){
        let phi = Phi{
            clauses: vec![
                Clause::new_c3(-2,3,-4),
                Clause::new_c3(-1,3,5),
                Clause::new_c3(-2,3,-5),
                Clause::new_c3(-1,4,-5),
                Clause::new_c3(-2,4,5),
                Clause::new_c3(-1,4,-5),
                Clause::new_c3(2,3,-4),
                Clause::new_c3(-1,-3,4),
                Clause::new_c3(-1,3,5),
                Clause::new_c3(-1,-2,4),
                Clause::new_c3(-1,3,-5),
                Clause::new_c3(1,-2,3)
            ]
        };
        let result = super::unrestrained_vector(&phi);
        println!("{:?}",result);
        assert!(result.contains(&0));
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(result.contains(&3));
        assert!(result.contains(&4));
    }

    #[test]
    fn solve(){
        let phi = Phi::from_file("TestData/test.cnf").unwrap();
        let result = super::solve(&phi);
        assert!(result.is_some());
        let result = result.unwrap();
        println!("{:?}",result);
    }

    #[test]
    fn solve_20()
    {
        let mut bad_results = 0;
        for _ in 0..100
        {
            let phi = Phi::from_file("TestData/solver20-0.cnf").unwrap();
            let result = super::solve(&phi);
            result.unwrap_or_else(||{bad_results+=1;vec![]});
        }
        println!("SuccessRate: {}", 100.0 - (bad_results as f64 / 100.0 * 100.0));
        assert_eq!(bad_results,0);
    }

    #[test]
    fn dpll()
    {
        let phi = Phi::from_file("TestData/test.cnf").unwrap();
        let result = super::dpll(&phi);
        assert!(result.is_some());
    }
}
