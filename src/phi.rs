
use std::{fs, collections::HashSet};
use crate::{clause::{Clause, Literal}, error::Error};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Phi
{
    pub clauses: Vec<Clause>,
}

impl std::fmt::Display for Phi
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f,"Phi {{")?;
        for c in &self.clauses
        {
            write!(f,"{}",c)?;
        }
        write!(f,"}}")?;
        Ok(())
    }
}

impl Phi{
    pub fn new() -> Phi
    {
        Phi{clauses: Vec::new()}
    }

    pub fn new_unsat() -> Phi
    {
        Phi{clauses: vec![Clause::Empty]}
    }

    pub fn vars(&self) -> usize
    {
        let mut ret: usize = 0;
        for c in &self.clauses
        {
            let max = c.max_literal();
            if max > ret { ret = max; }
        }
        ret + 1
    }

    pub fn reduce(&self, values: &[Option<bool>]) -> Phi
    {
        let mut ret = Phi::new();
        for c in &self.clauses
        {
            if let Some(new_clause) = c.reduce(values)
            {
                if let Clause::Empty = new_clause { return Phi::new_unsat(); }
                ret.clauses.push(new_clause);
            }
        }
        ret
    }

    pub fn find_unit(&self) -> Vec<Clause>
    {
        let mut ret: Vec<Clause> = Vec::new();
        for c in &self.clauses
        {
            if let Clause::C1(_) = c
            {
                ret.push(*c);
            }
        }
        ret
    }

    pub fn autoreduce_with_assignments(&self, assignments: &mut Vec<Option<bool>>) -> Phi
    {
        let units = self.find_unit();
        if units.len() == 0 {self.clone()}
        else 
        {
            for c in &units
            {
                if let Clause::C1(l) = c
                {
                    if assignments[l.index] != None { continue; }
                    assignments[l.index] = Some(l.value);
                }
            }
            let phi = self.reduce(&assignments);
            return phi.autoreduce_with_assignments(assignments);    
        }
    }

    pub fn autoreduce(&self) -> Phi
    {
        let units = self.find_unit();
        if units.len() == 0 {self.clone()}
        else 
        {
            let mut values: Vec<Option<bool>> = vec![None; self.vars()];
            for c in &units
            {
                if let Clause::C1(l) = c
                {
                    values[l.index] = Some(l.value);
                }
            }
            let phi = self.reduce(&values);
            return phi.autoreduce();    
        }
    }

    pub fn from_file(path: &str) -> Result<Phi,Box<dyn std::error::Error>>
    {
        let mut ret = Phi::new();
        //open file and read it
        let contents = fs::read_to_string(path)?;
        let clear_contents = contents.replace('\r',"");
        //split the file in lines
        let lines: Vec<&str> = clear_contents.split("\n").collect();
        let mut num_clauses: usize = 0;
        let mut num_vars: usize = 0;
        let mut inserted_clauses: usize = 0;
        //for each line
        for line in lines
        {
            //check if line is empty
            if line.len() == 0 { continue; }
            //check if line is a comment
            else if line.chars().nth(0).unwrap() == 'c' { continue; }
            //check if line is a problem line
            else if line.chars().nth(0).unwrap() == 'p' 
            {
                //split the line in words
                let words: Vec<&str> = line.split(" ").collect();
                //remove empty words
                let words: Vec<&str> = words.iter().filter(|x| x.len() > 0).map(|x| *x).collect();
                //problem is cnf
                assert_eq!(words[1], "cnf");
                //get the number of variables
                num_vars = words[2].parse()?;
                //get the number of clauses
                num_clauses = words[3].parse()?;
                //initialize the vector of clauses
                ret.clauses = Vec::with_capacity(num_clauses);
            }
            else 
            {
                let clause = Clause::from_str(line);
                match clause
                {
                    Ok(c) => {ret.clauses.push(c);},
                    Err(_) => {continue;}
                }
                inserted_clauses += 1;
            }
        }
        if num_clauses != inserted_clauses || num_vars != ret.vars() 
        {
            Err(Error::new("Error while parsing the file").into())
        }
        else
        {  
            Ok(ret) 
        }
    }

    pub fn eval(&self, assignment: &Vec<bool>) -> bool
    {
        for c in &self.clauses 
        {
            if !c.eval(assignment)
            {
                return false;
            }
        }
        true
    }

    pub fn phi_prime(&self, index: usize) -> Phi
    {
        //get the clauses where literal with index index is present
        let mut clauses: Vec<Clause> = Vec::new();
        for clause in &self.clauses{ if clause.contains(index){ clauses.push(*clause); } }
        Phi{clauses}
    }

    pub fn phi_second(&self, index: usize) -> Phi
    {
        //get the clauses where literal with index index is present
        //and remove the literal from the clause
        let mut clauses: Vec<Clause> = Vec::new();
        for clause in &self.clauses
        { 
            if clause.contains(index)
            { 
                let new_clause = clause.clone();
                let new_clause = new_clause.remove(index);
                clauses.push(new_clause); 
            } 
        }
        Phi{clauses}
    }

    pub fn create_unit(&self) -> Vec<Clause>
    {
        let var_count = self.vars();
        let mut fixed_vars: Vec<Result<bool,&str>> = vec![Err("not_init"); var_count];
        for c in &self.clauses
        {
            let literals = c.literals_vector();
            if literals.len() > 1
            {
                for l in &literals
                {
                    match fixed_vars[l.index]
                    {
                        Err(e) => {
                            match e {
                                "not_init" => {fixed_vars[l.index] = Ok(l.value);},
                                _ => {}
                            }
                        },
                        Ok(lit) => {
                            if lit != l.value {fixed_vars[l.index] = Err("not_fixed");}
                        }
                    }
                }
            }
        }
        let mut new_units = Vec::new();
        for (index,v) in fixed_vars.iter().enumerate()
        {
            match v {
                Ok(b) => {new_units.push(Clause::C1(Literal{index: index, value: *b}))},
                Err(_) => {}
            }
        }
        new_units
    }
    /// Create a new Phi with all the unit clauses derived from fixed variables
    pub fn add_unit(&self) -> Phi
    {
        let mut new_units = self.create_unit();
        let mut ret = Phi { clauses:  self.clauses.clone() };
        ret.clauses.append(&mut new_units);
        ret
    }

    pub fn get_variables(&self) -> Vec<usize>{
        let mut variables: HashSet<usize> = HashSet::new();
        for clause in &self.clauses{
            for literal in clause.get_variables(){
                if !variables.contains(&literal){
                    variables.insert(literal);
                }
            }
        }
        let variables: Vec<usize> = variables.into_iter().collect();
        variables
    }

}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn from_file()
    {
        let phi = Phi::from_file("TestData/test.cnf").unwrap();
        assert_eq!(phi.clauses.len(), 3);
        assert_eq!(phi.clauses[0], Clause::new_c3(1,2,3));
        assert_eq!(phi.clauses[1], Clause::new_c3(-1,-2,3));
        assert_eq!(phi.clauses[2], Clause::new_c3(1,-2,3));
        let phi = Phi::from_file("TestData/test2.cnf");
        assert!(phi.is_err());
    }
    #[test]
    fn phi_prime()
    {
        let c1 = Clause::new_c3(1,2,3);
        let c2 = Clause::new_c3(-2,-3,4);
        let c3 = Clause::new_c3(1,-2,3);
        let phi = Phi{clauses: vec![c1,c2,c3]};
            
        let phi_p = phi.phi_prime(0);
        assert_eq!(phi_p.clauses[0], c1);
        assert_eq!(phi_p.clauses[1], c3);
        assert_eq!(phi_p.clauses.len(), 2);
    }

    #[test]
    fn create_unit()
    {
        let c1 = Clause::new_c3(1,2,3);
        let c2 = Clause::new_c3(2,3,4);
        let c3 = Clause::new_c3(1,-2,-3);
        let phi = Phi{clauses: vec![c1,c2,c3]};
        let units = phi.create_unit();
        assert_eq!(units.len(), 2);
        assert_eq!(units[0], Clause::new_c1(1));
        assert_eq!(units[1], Clause::new_c1(4));
    }
}