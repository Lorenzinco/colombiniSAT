use std::collections::HashMap;
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
    let lit = Literal{index: lit.index, value: lit.value, implicated: false, assigned: true};
    return graph.add_node(lit);
}



pub fn update_implication_graph(phi: &Phi, graph:&mut DiGraph<Literal,Literal>){
    let implications = phi.get_implications();

    for clause in implications{
        match clause{
            Clause::C3(lit1,lit2,lit3)=>{
                if !lit1.assigned{
                    let l1 = Literal{index: lit1.index, value: lit1.value, implicated: true, assigned: false};
                    add_implication(lit2, Some(lit3), l1, graph);
                }
                if !lit2.assigned{
                    let l2 = Literal{index: lit2.index, value: lit2.value, implicated: true, assigned: false};
                    add_implication(lit1, Some(lit3), l2, graph);
                }
                if !lit3.assigned{
                    let l3 = Literal{index: lit3.index, value: lit3.value, implicated: true, assigned: false};
                    add_implication(lit1, Some(lit2), l3, graph);
                }
            }
            Clause::C2(lit1,lit2)=>{
                if !lit1.assigned{
                    let l1 = Literal{index: lit1.index, value: lit1.value, implicated: true, assigned: false};
                    add_implication(lit2, None, l1, graph);
                }
                if !lit2.assigned{
                    let l2 = Literal{index: lit2.index, value: lit2.value, implicated: true, assigned: false};
                    add_implication(lit1, None, l2, graph);
                }
            }
            _=>{unreachable!()}
        }
    }
}

pub fn find_conflicts(graph :&DiGraph<Literal,Literal>)->Vec<Literal>{
    let mut literals_found: HashMap<usize, bool> = HashMap::new(); 
    let mut conflicts: Vec<Literal> = Vec::new();
    for node in graph.node_weights(){
        if node.implicated{
            if literals_found.contains_key(&node.index){
                conflicts.push(*node);
            }
            else{
                literals_found.insert(node.index,true);
            }
        }
    }
    conflicts
}

///add an implication a&b -> c to the graph
/// if b is none, adds a -> c to the graph
    /// # Arguments
    /// * `graph` - A mutable reference to a DiGraph<Literal,Literal>
    /// * `a` - The first literal
    /// * `b` - The optional second literal
    /// * `c` - The implicated literal
pub fn add_implication(a: Literal, b: Option<Literal>, c: Literal, graph: &mut DiGraph<Literal,Literal>)->(NodeIndex,Option<NodeIndex>,NodeIndex){
    
    if !graph.node_weights().any(|x| (x.index == a.index && x.value == a.value)){
        graph.add_node(a);
    }

    if b.is_some(){
        let _b= b.unwrap();
        if !graph.node_weights().any(|x| (x.index == _b.index && x.value == _b.value)){
            graph.add_node(_b);
        }
    }
    if !graph.node_weights().any(|x| (x == &c && x.value == c.value)){
        graph.add_node(c);
    }

    let node1 = graph.node_indices().find(|i| (graph[*i].index,graph[*i].value)== (a.index,a.value)).unwrap();
    let mut node2: Option<NodeIndex> = None;
    if b.is_some(){let _b = b.unwrap();node2 = Some(graph.node_indices().find(|i| (graph[*i].index,graph[*i].value)== (_b.index,_b.value)).unwrap());}
    let node3 = graph.node_indices().find(|i| (graph[*i].index,graph[*i].value) == (c.index,c.value)).unwrap();

    if node2.is_some(){
        graph.add_edge(node2.unwrap(),node3,Literal{index:0,value:false,implicated:false, assigned: false});
    }
    graph.add_edge(node1,node3,Literal{index:0,value:false,implicated:false, assigned: false});
    (node1,node2,node3)
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
                                let clause = Clause::C1(Literal{index: i,value:*l1,implicated:true, assigned: false});
                                //phi.clauses.push(clause);
                                phi.update_implications(&clause);
                                assignment[i] = Some(*l1);
                                added_unit_clause = true;
                            },
                            (l1,l2) => {
                                match l1{
                                    Some(l1) => {
                                        let clause = Implication{
                                            from: Literal{index: literal, value: true,implicated: true, assigned: false}, 
                                            to: Literal{index: i, value: *l1, implicated: true, assigned: false}
                                        }.to_clause();
                                        //phi.clauses.push(clause);
                                    }
                                    None => {}
                                }
                                match l2{
                                    Some(l2) => {
                                        let clause = Implication{
                                            from: Literal{index: literal, value: false,implicated: true, assigned: false}, 
                                            to: Literal{index: i, value: *l2, implicated:true, assigned: false}
                                        }.to_clause();
                                        //phi.clauses.push(clause);
                                    }
                                    None => {}
                                }
                            }
                        }
                    }
                },
                (Ok(solution_t),Err(_)) => {
                    let clause = Clause::C1(Literal{index: literal,value:true, implicated: true, assigned: false});
                    //phi.clauses.push(clause);
                    phi.update_implications(&clause);
                    assignment[literal] = Some(true);
                    added_unit_clause = true;
                    for (index,value) in solution_t.iter().enumerate(){
                        match value{
                            Some(value) => {
                                let clause = Implication{
                                    from: Literal{index: literal, value: true, implicated: true, assigned: false}, 
                                    to: Literal{index: index, value: *value, implicated:true, assigned: false}
                                }.to_clause();
                                //phi.clauses.push(clause);
                            }
                            None => {}
                        }
                    }
                },
                (Err(_),Ok(solution_f)) => {
                    let clause = Clause::C1(Literal{index: literal, value: false, implicated: true, assigned: false});
                    //phi.clauses.push(clause);
                    phi.update_implications(&clause);
                    assignment[literal] = Some(false);
                    added_unit_clause = true;
                    for (index,value) in solution_f.iter().enumerate(){
                        match value{
                            Some(value) => {
                                let clause = Implication{
                                    from: Literal{index: literal, value: false, implicated:false, assigned: false}, 
                                    to: Literal{index: index, value: *value, implicated: true, assigned: false}
                                }.to_clause();
                                //phi.clauses.push(clause);
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
    use crate::phi;
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
        let x1 = Literal{index: 0, value: true, implicated: false, assigned: false};
        let x2 = Literal{index: 1, value: true, implicated: false, assigned: false};
        let x3 = Literal{index: 2, value: true, implicated: false, assigned: false};
        let x4 = Literal{index: 3, value: true, implicated: false, assigned: false};

        add_decision(x1,&mut graph);
        add_decision(x2,&mut graph);

        add_implication(x1,None,x3,&mut graph);
        add_implication(x2,None,x3,&mut graph);

        let (n3,_,_) = add_implication(x3,None,x4,&mut graph);

        println!("{:?}",graph);
        assert_eq!(graph.node_count(),4);
        assert_eq!(graph.edge_count(),3);
        assert_eq!(graph.neighbors_directed(n3,Incoming).count(),2)
    }

    #[test]
    fn update_implications(){
        let l1 = Literal{index: 0, value: true, implicated: false, assigned: true};
        let l2 = Literal{index: 1, value: true, implicated: false, assigned: true};
        let l3 = Literal{index: 2, value: true, implicated: false, assigned: false};
        let l4 = Literal{index: 3, value: true, implicated: false, assigned: true};

        let c1 = Clause::C3(l1,l2,l3);
        let c2 = Clause::C2(l3,l4);
        let phi = Phi{clauses: vec![c1,c2]};
        let mut graph = DiGraph::<Literal,Literal>::new();

        update_implication_graph(&phi,&mut graph);
        assert_eq!(graph.edge_count(),3);
    }

    #[test]
    fn find_conflict(){
        let l1 = Literal{index: 0, value: true, implicated: false, assigned: true};
        let l2 = Literal{index: 1, value: true, implicated: false, assigned: true};
        let l3 = Literal{index: 2, value: true, implicated: false, assigned: false};
        let l4 = Literal{index: 3, value: true, implicated: false, assigned: false};
        let nl3 = Literal{index: 2, value: false, implicated: false, assigned: false};
        let nl4 = Literal{index: 3, value: false, implicated: false, assigned: false};

        let c1 = Clause::C3(l1,l2,l3);
        let c2 = Clause::C3(l1,l2,nl3);
        let c3 = Clause::C2(l1,l4);
        let c4 = Clause::C2(l2,nl4);
        let phi = Phi{clauses: vec![c1,c2,c3,c4]};
        let mut graph = DiGraph::<Literal,Literal>::new();

        update_implication_graph(&phi,&mut graph);
        let conflicts = find_conflicts(&graph);

        assert_eq!(conflicts.len(),2);
        assert_eq!(conflicts[0].index,l3.index);
        assert_eq!(conflicts[1].index,l4.index);
    }
}
