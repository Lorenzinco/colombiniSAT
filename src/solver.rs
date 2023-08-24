use std::vec;
use std::error;

use petgraph::{graph::DiGraph,stable_graph::NodeIndex,Direction::Incoming};

use crate::{phi::Phi, error::Error, clause::{Clause, Literal, Implication}, two_satisfiability::solve_2_sat};

/*
Core idea: at each step identify the i-th literal that is forced to be true or false within their i-th phi_prime
phi_prime_i being a subset of phi, composed by the clauses that contains the literal i.
*/

pub struct Solver{
    pub phi: Phi,
    pub solution: Option<Vec<bool>>
}

impl Solver{
    ///Creates a Solver object from a path to a dmacs file\
        /// # Arguments
        /// * `dmacs_path` - A string slice that holds the path to the dmacs file
        /// # Returns
        /// * A Solver object or an Error if the path to the file is incorrect.
        /// # Example
        /// ```
        /// use colombini_sat::solver::Solver;
        /// 
        /// let solver = Solver::create("TestData/test.cnf").unwrap();
        /// ```
    pub fn create(dmacs_path: &str)->Result<Solver,Box<dyn error::Error>>{
        
        match Phi::from_file(dmacs_path){
            Ok(phi) => Ok(Solver{phi: phi, solution: None}),
            Err(e) => Err(e)
        }
    }

    ///Returns the number of literals in the formula
    pub fn num_variables(&self)->usize{
        self.phi.vars()
    }

    ///Returns the number of clauses in the formula
    pub fn num_clauses(&self)->usize{
        self.phi.clauses.len()
    }


    ///Returns a satisfying assignment for the formula if it exists, None otherwise
    /// # Example
    /// ```
    /// use colombini_sat::solver::Solver;
    /// 
    /// let solver = Solver::create("TestData/test.cnf").unwrap();
    /// let solution = solver.solve();
    /// match solution{
    ///    Some(solution) => {println!("SAT({:?})",solution);},
    ///   None => {println!("UNSAT");}
    /// }
    /// ```
    pub fn solve(&self)->Option<Vec<isize>>{
        let solution = solve(&self.phi);
        match solution{
            Some(solution) => {
                let mut assignment: Vec<isize> = vec![0;self.phi.vars()];
                for (index,value) in solution.iter().enumerate(){
                    if *value{
                        assignment[index] = index as isize +1;
                    }
                    else{
                        assignment[index] = -(index as isize +1);
                    }
                }
                return Some(assignment);
            },
            None => None
        }
    }
}

/* 
fn conflict_to_clauses(conflict: &Vec<Literal>)->Vec<Clause>{
    //TODO
}
*/
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

///add a decision literal to the graph
    /// # Arguments
    /// * `graph` - A mutable reference to a DiGraph<Literal,Literal>
    /// * `lit` - The literal to add
pub fn add_decision(lit: Literal, graph: &mut DiGraph<Literal,Literal>)->NodeIndex{
    return graph.add_node(lit);
}

///add an implication a -> b to the graph
    /// # Arguments
    /// * `graph` - A mutable reference to a DiGraph<Literal,Literal>
    /// * `a` - The first literal
    /// * `b` - The second literal
pub fn add_implication(a: Literal, b: Literal, graph: &mut DiGraph<Literal,Literal>)->(NodeIndex,NodeIndex){
    if !graph.node_weights().any(|x| x == &a){
        graph.add_node(b);
    }
    if !graph.node_weights().any(|x| x == &b){
        graph.add_node(b);
    }

    let node1 = graph.node_indices().find(|i| graph[*i] == a).unwrap();
    let node2 = graph.node_indices().find(|i| graph[*i] == b).unwrap();
    graph.add_edge(node1,node2,Literal{index:0,value:false,implicated:false});
    (node1,node2)
}

pub fn solve(phi: &Phi) -> Option<Vec<bool>>
{
    let n_vars = phi.vars();
    let mut assignment: Vec<Option<bool>> = vec![None;n_vars];
    let mut phi = phi.clone();

    while phi.clauses.len() > 0{
        phi = phi.autoreduce_with_assignments(&mut assignment);
        //check if phi is empty
        if phi.clauses.len() > 0 && phi.clauses[0] == Clause::Empty{
            return None;
        }

        let mut added_unit_clause: bool = false;
        //for each variable, check if it is forced to be true or false
        let literals = phi.get_variables();
        //for every literal in literals that isnt in the reserve list
        for literal in literals{
            let phi_prime = phi.phi_prime(literal);
            assignment[literal] = Some(true);
            let phi_prime_true: Phi = phi_prime.reduce(&assignment);
            let solution_true: Result<Vec<Option<bool>>, Error> = solve_2_sat(&phi_prime_true,n_vars);

            assignment[literal] = Some(false);
            let phi_prime_false: Phi = phi_prime.reduce(&assignment);
            let solution_false: Result<Vec<Option<bool>>, Error> = solve_2_sat(&phi_prime_false,n_vars);
            assignment[literal] = None;
            match (solution_true,solution_false){
                (Ok(solution_t),Ok(solution_f)) => {
                    for (i,(lit_t,lit_f)) in solution_t.iter().zip(solution_f.iter()).enumerate(){
                        match (lit_t,lit_f){
                            (Some(l1),Some(l2)) if l1==l2 => {
                                let clause = Clause::C1(Literal{index: i,value:*l1,implicated:true});
                                phi.clauses.push(clause);
                                added_unit_clause = true;
                            },
                            (l1,l2) => {
                                match l1{
                                    Some(l1) => {
                                        let clause = Implication{
                                            from: Literal{index: literal, value: true,implicated: true}, 
                                            to: Literal{index: i, value: *l1, implicated: true}
                                        }.to_clause();
                                        phi.clauses.push(clause);
                                    }
                                    None => {}
                                }
                                match l2{
                                    Some(l2) => {
                                        let clause = Implication{
                                            from: Literal{index: literal, value: false,implicated: true}, 
                                            to: Literal{index: i, value: *l2, implicated:true}
                                        }.to_clause();
                                        phi.clauses.push(clause);
                                    }
                                    None => {}
                                }
                            }
                        }
                    }
                },
                (Ok(solution_t),Err(_)) => {
                    let clause = Clause::C1(Literal{index: literal,value:true, implicated: true});
                    phi.clauses.push(clause);
                    added_unit_clause = true;
                    for (index,value) in solution_t.iter().enumerate(){
                        match value{
                            Some(value) => {
                                let clause = Implication{
                                    from: Literal{index: literal, value: true, implicated: true}, 
                                    to: Literal{index: index, value: *value, implicated:true}
                                }.to_clause();
                                phi.clauses.push(clause);
                            }
                            None => {}
                        }
                    }
                },
                (Err(_),Ok(solution_f)) => {
                    let clause = Clause::C1(Literal{index: literal, value: false, implicated: true});
                    phi.clauses.push(clause);
                    added_unit_clause = true;
                    for (index,value) in solution_f.iter().enumerate(){
                        match value{
                            Some(value) => {
                                let clause = Implication{
                                    from: Literal{index: literal, value: false, implicated:false}, 
                                    to: Literal{index: index, value: *value, implicated: true}
                                }.to_clause();
                                phi.clauses.push(clause);
                            }
                            None => {}
                        }
                    }
                },
                (Err(_),Err(_)) => {
                    return None;
                },
            }
        }
        if !added_unit_clause
        {
            //if no literal is forced to be true or false, choose one and backtrack
            let literals = phi.get_variables();
            if literals.len() > 0{
                let literal = literals[0];
                assignment[literal] = Some(true);
                let phi_true = phi.reduce(&assignment);
                let result_true = solve(&phi_true);
                match result_true{
                    Some(_) => {
                        //merge result true with assignment
                        for (index,value) in result_true.unwrap().into_iter().enumerate(){
                            
                            assignment[index] = Some(value);
                                
                        }
                        return Some(assignment.iter().map(|x| x.unwrap_or(false)).collect());
                    },
                    None => {
                        assignment[literal] = Some(false);
                        let phi_false = phi.reduce(&assignment);
                        let result_false = solve(&phi_false);
                        match result_false{
                            Some(_) => {
                                //merge result false with assignment
                                for (index,value) in result_false.unwrap().into_iter().enumerate(){
                                    assignment[index] = Some(value);
                                }
                                return Some(assignment.iter().map(|x| x.unwrap_or(false)).collect());
                            },
                            None => {
                                return None;
                            }
                        }
                    }
                }
            }
        }
    }
    //return the assignment vector, if some value is still none fill it with false
    Some(assignment.iter().map(|x| x.unwrap_or(false)).collect())
}




#[cfg(test)]
mod tests
{
    use crate::phi::*;
    use crate::solver::*;
    

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

    #[test]
    fn create_implications(){
        let mut graph = DiGraph::<Literal,Literal>::new();
        let x1 = Literal{index: 0, value: true, implicated: false};
        let x2 = Literal{index: 1, value: true, implicated: false};
        let x3 = Literal{index: 2, value: true, implicated: false};
        let x4 = Literal{index: 3, value: true, implicated: false};

        add_decision(x1,&mut graph);
        add_decision(x2,&mut graph);

        add_implication(x1,x3,&mut graph);
        add_implication(x2,x3,&mut graph);

        let (n3,_) = add_implication(x3,x4,&mut graph);

        assert_eq!(graph.node_count(),4);
        assert_eq!(graph.edge_count(),3);
        assert_eq!(graph.neighbors_directed(n3,Incoming).count(),2)
    }
}
