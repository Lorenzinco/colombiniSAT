use std::collections::HashMap;
use petgraph::{graph::{Graph, NodeIndex}, algo, prelude::DiGraph};
use crate::{phi::Phi, error::Error, clause::Clause};



fn update_active_implications(solution: &mut Vec<bool>,problem: &Vec<(isize,isize)>,active_implications: &mut Vec<(isize,isize)>){
    active_implications.clear();
    for (i,xi_true) in solution.iter().enumerate(){
        if *xi_true{
            for (a,b) in problem.iter(){
                if a.abs() == i as isize{
                    active_implications.push((a.clone(),b.clone()));
                }
            }
        }
    }
}

fn determine_the_xj_that_have_just_become_active_and_inactive(active_implications: &mut Vec<(isize,isize)>,active_literals: &mut Vec<usize>)->(Vec<usize>,Vec<usize>){
    let mut active = Vec::new();
    let mut inactive = Vec::new();
    for j in active_literals.iter(){
        let mut active_j = false;
        for (_a,b) in active_implications.iter(){
            if b.abs() == *j as isize{
                active_j = true;
            }
        }
        if !active_j{
            inactive.push((*j).clone());
        }
    }

    for (_a,b) in active_implications.iter(){
        let _b = b.abs() as usize;
        if !active_literals.contains(&_b){
            active.push(_b.clone());
        }
    }
    (active, inactive)
}

fn remove_from_active_list_and_remember_predecessor(active_literals: &mut Vec<usize>, just_become_inactive: &Vec<usize>, n: &mut Vec<usize>){
    let mut to_remove: Vec<usize> = Vec::new();
    for j in just_become_inactive.iter(){
        for i in active_literals.iter(){
            if *i == *j{
                to_remove.push(*i);
                n.push(*i);
            }
        }
    }
    for i in to_remove.iter(){
        active_literals.remove(*i);
    }


}

pub fn create_graph(phi: &Phi)->Option<DiGraph<isize,isize>>{
    let mut graph = DiGraph::<isize,isize>::new();
    let mut added_literals = HashMap::<isize,NodeIndex>::new();

    //remove unit clauses
    let mut assignments: Vec<Option<bool>> = vec![None;phi.vars()];
    let rphi = phi.autoreduce_with_assignments(&mut assignments);
    //check that all clauses in phi are 2-sat and
    //for each clause in phi, create a graph composed by each literal being a node 
    //and each clause impling two edges between the two literals in the clause
    for clause in &rphi.clauses
    {
        match clause
        {
            Clause::C3(_,_,_) => { panic!("Not a 2-sat formula")},
            Clause::C2(l1,l2) => {
                for i in 0..2
                {
                    let index1: isize = if i == 0 { -l1.as_isize() } else { -l2.as_isize() };
                    let index2: isize = if i == 0 { l2.as_isize() } else { l1.as_isize() };

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
                    graph.add_edge(n1.unwrap(), n2.unwrap(), 0);
                }
            },
            Clause::C1(_) => { unreachable!() },
            Clause::Empty => { return None },
        }
    }
    Some(graph)
}

pub fn enumerate(og_solution: &Vec<Option<bool>>, phi_prime: &Phi){
    //create a new phi that has all the literals that are true in solution inverted
    let mut _added_to_graph = HashMap::<usize,NodeIndex>::new();
    let mut solution = og_solution.clone();
    let mut phi = phi_prime.clone();
    let mut permutation_map = HashMap::<usize,usize>::new();
    let mut literals = phi.get_variables();
    literals.sort();

    let _graph = create_graph(&phi);

    for (i,literal) in literals.iter().enumerate(){
        permutation_map.insert(i, literal.clone());
        match solution[*literal]{
            Some(true) => {phi.invert_literal(i);solution[*literal]=Some(false)},
            Some(false) => {},
            None => {}
        }
    }

    let problem = Graph::<isize,isize>::new();
    let mut _sorted: Vec<NodeIndex> = Vec::new();
    match algo::toposort(&problem, None){
        Ok(result) => {_sorted = result;},
        Err(_) => {panic!("Not a DAG")}
    }


}

pub fn _enumerate(solution: &mut Vec<bool>, active_literals: &mut Vec<usize>,active_implications: &mut Vec<(isize,isize)>, index: usize, depth: usize, solutions: &mut Vec<Vec<bool>>,problem: &Vec<(isize,isize)>){
    let i = index;
    solution[i]=true;
    if depth % 2 == 0 {solutions.push(solution.clone());}
    update_active_implications(solution, problem, active_implications);
    let (mut m, inactive) = determine_the_xj_that_have_just_become_active_and_inactive(active_implications, active_literals);
    let mut n: Vec<usize> = Vec::new();
    remove_from_active_list_and_remember_predecessor(active_literals, &inactive, &mut n);
    m.sort();
    //merge m and active literals in order
    for j in m.iter().rev(){
        if *j==i {break};
        active_literals.push(*j);
        active_literals.sort();
        _enumerate(solution, active_literals, active_implications, *j, depth+1, solutions, problem);
    }
    if depth %2 == 1 {solutions.push(solution.clone());}
    solution[i]=false;
    for j in m.iter(){
        active_literals.push(*j);
    }
    active_literals.sort();
    update_active_implications(solution, problem, active_implications);
}

pub fn _solve_2_sat(phi: &Phi) -> Result<Vec<Option<bool>>, Error>
{
    let mut graph = Graph::<isize,i8>::new();
    let mut added_literals = HashMap::<isize,NodeIndex>::new();

    //remove unit clauses
    let mut assignments: Vec<Option<bool>> = vec![None;phi.vars()];
    let rphi = phi.autoreduce_with_assignments(&mut assignments);
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
    let mut sccs = algo::tarjan_scc(&graph);
    sccs.reverse();
    //explore the groups and find if there is a node with a completely connected (two way) path to its negation
    /*the groups are returned in reverse topological order so, a satisfyng assignment 
    can be found by assigning the variables in the reverse order of the groups*/
    for scc in sccs
    {
        for node in &scc
        {
            let index: isize = graph[*node];
            let mut arr_index = index.abs() as usize;
            arr_index -= 1;
            match assignments[arr_index]{
                Some(_) => {},
                None => {
                if index > 0{
                    assignments[arr_index] = Some(true);
                }else{
                    assignments[arr_index] = Some(false);
                }}
            }
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

    let mut phi_ = rphi.clone();
    let mut new_assignments = assignments.clone();
    for (index,assignment) in assignments.iter().enumerate(){
        match assignment{
            Some(true) => {
                phi_.invert_literal(index);
                new_assignments[index] = Some(false);
            },
            Some(false) => {},
            None => {}
        }
    }

    println!("phi = {}",rphi);
    print!("assignments = ");
    for (i, a) in assignments.iter().enumerate() {
        let index = i + 1;
        match a {
            Some(true) => print!("X_{index} = T, "),
            Some(false) => print!("X_{index} = F, "),
            None => (),
        }
    }
    println!();
    println!("phi_ = {}",phi_);
    print!("new_assignments = ");
    for (i, a) in new_assignments.iter().enumerate() {
        let index = i + 1;
        match a {
            Some(true) => print!("X_{index} = T, "),
            Some(false) => print!("X_{index} = F, "),
            None => (),
        }
    }
    println!();

    let mut assignments: Vec<Option<bool>> = vec![None;phi.vars()];

    for clause in &phi_.clauses
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

    let mut sccs = algo::tarjan_scc(&graph);
    sccs.reverse();
    //explore the groups and find if there is a node with a completely connected (two way) path to its negation
    /*the groups are returned in reverse topological order so, a satisfyng assignment 
    can be found by assigning the variables in the reverse order of the groups*/
    for scc in sccs
    {
        for node in &scc
        {
            let index: isize = graph[*node];
            let mut arr_index = index.abs() as usize;
            arr_index -= 1;
            match assignments[arr_index]{
                Some(_) => {},
                None => {
                if index > 0{
                    assignments[arr_index] = Some(true);
                }else{
                    assignments[arr_index] = Some(false);
                }}
            }
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

    print!("After shit algorithm: ");
    for (i, a) in assignments.iter().enumerate() {
        let index = i + 1;
        match a {
            Some(true) => print!("X_{index} = T, "),
            Some(false) => print!("X_{index} = F, "),
            None => (),
        }
    }
    println!();
    
    Ok(assignments)
}

pub fn solve_2_sat(phi: &Phi,n:usize) -> Result<Vec<Option<bool>>, Error>
{
    //remove unit clauses
    let mut assignments: Vec<Option<bool>> = vec![None;n];
    let rphi = phi.autoreduce_with_assignments(&mut assignments);

    //create implication graph
    let implications = create_graph(&rphi);
    let graph:DiGraph<isize,isize>;

    match implications{
        Some(g)=>{graph = g}
        None => {return Err(Error::new("Not satisfiable, found empty clause in 2-sat"))}
    }

    //find all completely connected groups
    let mut sccs: Vec<Vec<NodeIndex>> = algo::tarjan_scc(&graph);
    sccs.reverse();
    //explore the groups and find if there is a node with a completely connected (two way) path to its negation
    /*the groups are returned in reverse topological order so, a satisfyng assignment 
    can be found by assigning the variables in the reverse order of the groups*/
    for scc in sccs
    {
        let mut added_literals = HashMap::<isize,NodeIndex>::new();
        for node in &scc
        {
            let index: isize = graph[*node];
            added_literals.insert(index, *node);
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

    Ok(assignments)
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
        let result_true = solve_2_sat(&phi_prime_true,var_count).is_ok();
        let result_false = solve_2_sat(&phi_prime_false,var_count).is_ok();
        if result_true && result_false{
            reserves.push(*index);
        }
    } 
    reserves
}

#[cfg(test)]
mod tests{
    use crate::{phi::Phi, clause::Clause, two_satisfiability::create_graph};

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
        let result = super::solve_2_sat(&phi,phi.vars());
        assert!(result.is_ok());

        let phi = Phi{
            clauses: vec![
                Clause::new_c2(1,-2),
                Clause::new_c2(1,2),
                Clause::new_c2(-1,2),
                Clause::new_c2(-1,-2),
                Clause::new_c2(1,-3),
            ]
        };
        let result = super::solve_2_sat(&phi,phi.vars());
        assert!(result.is_err());
    }

    #[test]
    fn graph(){
        let phi = Phi{
            clauses: vec![
                Clause::new_c2(1,-2),
                Clause::new_c2(-1,2),
                Clause::new_c2(-1,-2),
                Clause::new_c2(1,-3),
            ]
        };

        let graph = create_graph(&phi);
        println!("{:?}",graph);



    }

}