use crate::error::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Literal
{
    pub index: usize, 
    //the value that the literal must assume to become true
    pub value: bool,
    pub implicated: bool
}

impl Literal
{
    pub fn as_isize(&self) -> isize
    {
        if self.value { self.index as isize + 1 }
        else { -(self.index as isize + 1) }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Implication
{
    pub from: Literal,
    pub to: Literal
}

impl Implication
{
    /// USE 1-BASED INDEXING
    pub fn new(from: isize, to: isize) -> Implication
    {
        Implication{from: Literal{index: from.abs() as usize - 1, value: from > 0, implicated: false}, 
                    to: Literal{index: to.abs() as usize - 1, value: to > 0, implicated: true}}
    }

    pub fn to_clause(&self) -> Clause
    {
        let mut l1 = self.from;
        let l2 = self.to;
        l1.value = !l1.value;
        Clause::C2(l1,l2)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clause 
{
    C3(Literal, Literal, Literal),
    C2(Literal, Literal),
    C1(Literal),
    Empty
}

impl std::fmt::Display for Clause
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Clause::C3(l1, l2, l3) => write!(f,"({} {} {})",l1.as_isize(),l2.as_isize(),l3.as_isize()),
            Clause::C2(l1, l2) => write!(f,"({} {})",l1.as_isize(),l2.as_isize()),
            Clause::C1(l1) => write!(f,"({})",l1.as_isize()),
            Clause::Empty => write!(f,"(Empty)")
        }
    }
}

impl Clause
{
    // USE 1-BASED INDEXING
    pub fn new_c3(v1: isize, v2: isize, v3: isize) -> Clause
    {
        assert_ne!(v1, 0);
        assert_ne!(v2, 0);
        assert_ne!(v3, 0);

        Clause::C3(Literal{index: v1.abs() as usize - 1, value: v1 > 0, implicated: false}, 
                   Literal{index: v2.abs() as usize - 1, value: v2 > 0, implicated: false}, 
                   Literal{index: v3.abs() as usize - 1, value: v3 > 0, implicated: false})
    }

    // USE 1-BASED INDEXING
    pub fn new_c2(v1: isize, v2: isize) -> Clause
    {
        assert_ne!(v1, 0);
        assert_ne!(v2, 0);

        Clause::C2(Literal{index: v1.abs() as usize - 1, value: v1 > 0, implicated: false}, 
                   Literal{index: v2.abs() as usize - 1, value: v2 > 0, implicated: false})
    }

    // USE 1-BASED INDEXING
    pub fn new_c1(v1: isize) -> Clause
    {
        assert_ne!(v1, 0);

        Clause::C1(Literal{index: v1.abs() as usize - 1, value: v1 > 0, implicated: false})
    }

    pub fn new_c1_implicated(v1: isize) -> Clause
    {
        assert_ne!(v1, 0);

        Clause::C1(Literal{index: v1.abs() as usize - 1, value: v1 > 0, implicated: true})
    }

    pub fn get_literal(&self, index: usize) -> Option<Literal>
    {
        match self
        {
            Clause::C3(l1, l2, l3) => 
            {
                if l1.index == index { Some(*l1) }
                else if l2.index == index { Some(*l2) }
                else if l3.index == index { Some(*l3) }
                else { None }
            },
            Clause::C2(l1, l2) => 
            {
                if l1.index == index { Some(*l1) }
                else if l2.index == index { Some(*l2) }
                else { None }
            },
            Clause::C1(l1) => 
            {
                if l1.index == index { Some(*l1) }
                else { None }
            },
            Clause::Empty => None
        }
    }

    pub fn is_implicated(&self)->bool{
        match self
        {
            Clause::C2(l1, l2) => 
            {
                if l1.implicated || l2.implicated {
                    return true;
                }
            },
            Clause::C1(l1) => 
            {
                if l1.implicated {
                    return true;
                }
            },
            _ => {}
        }
        false
    }

    pub fn literals_vector(&self) -> Vec<Literal>
    {
        match self
        {
            Clause::C3(l1, l2, l3) => vec![*l1, *l2, *l3],
            Clause::C2(l1, l2) => vec![*l1, *l2],
            Clause::C1(l1) => vec![*l1],
            Clause::Empty => Vec::new()
        }
    }

    pub fn get_variables(&self) -> Vec<usize>
    {
        match self
        {
            Clause::C3(l1, l2, l3) => vec![l1.index, l2.index, l3.index],
            Clause::C2(l1, l2) => vec![l1.index, l2.index],
            Clause::C1(l1) => vec![l1.index],
            Clause::Empty => Vec::new()
        }
    }

    pub fn from_str(s: &str) -> Result<Clause,Box<dyn std::error::Error>>
    {
        //split the line in words
        let new_s = s.trim();
        let words: Vec<&str> = new_s.split(" ").collect();
        if words.len() > 4 || words.len() < 2
        {
            Err(Error::new("Clause is not 3-SAT or empty").into())
        }
        else if words.len() == 2
        {
            let (v1, v2) = 
                (words[0].parse::<isize>()?,
                words[1].parse::<isize>()?);
            if v2 != 0 { Err(Error::new("Error while parsing clause").into()) }
            else if v1 == 0 { Err(Error::new("Error while parsing clause").into()) }
            else { Ok(Clause::new_c1(v1)) }
        }
        else if words.len() == 3
        {
            //convert the words to numbers
            let (v1, v2, v3) = 
                (words[0].parse::<isize>()?,
                words[1].parse::<isize>()?, 
                words[2].parse::<isize>()?);
            // v4 must be 0
            if v3 != 0 { Err(Error::new("Error while parsing clause").into()) }
            else if v1 == 0 || v2 == 0 { Err(Error::new("Error while parsing clause").into()) }
            else { Ok(Clause::new_c2(v1,v2)) }
        }
        else 
        {
            //convert the words to numbers
            let (v1, v2, v3, v4) = 
                (words[0].parse::<isize>()?,
                words[1].parse::<isize>()?, 
                words[2].parse::<isize>()?, 
                words[3].parse::<isize>()?);
            // v4 must be 0
            if v4 != 0 { Err(Error::new("Error while parsing clause").into()) }
            // v1, v2, v3 must not be 0
            else if v1 == 0 || v2 == 0 || v3 == 0 { Err(Error::new("Error while parsing clause").into()) }
            else { Ok(Clause::new_c3(v1,v2,v3)) }
        }

    }
    pub fn max_literal(&self) -> usize 
    {
        match self
        {
            Clause::C3(l1,l2,l3) => 
            {
                if l1.index > l2.index 
                {
                    if l1.index > l3.index { l1.index } else { l3.index }
                }
                else
                {
                    if l2.index > l3.index { l2.index } else { l3.index }
                }
            }, 
            Clause::C2(l1,l2) => 
            {
                if l1.index > l2.index { l1.index } else { l2.index }
            },
            Clause::C1(l1) => 
            {
                l1.index
            },
            Clause::Empty => 
            {
                0
            }
        }
    }

    pub fn eval(&self, values: &[bool]) -> bool
    {
        match self
        {
            Clause::C3(l1,l2,l3) => 
            {
                (if l1.value == true { values[l1.index] } else { !values[l1.index] })
                ||
                (if l2.value == true { values[l2.index] } else { !values[l2.index] })
                ||
                (if l3.value == true { values[l3.index] } else { !values[l3.index] })
            }, 
            Clause::C2(l1,l2) => 
            {
                (if l1.value == true { values[l1.index] } else { !values[l1.index] })
                ||
                (if l2.value == true { values[l2.index] } else { !values[l2.index] })
            },
            Clause::C1(l1) => 
            {
                if l1.value == true { values[l1.index] } else { !values[l1.index] }
            },
            Clause::Empty => 
            {
                false
            }
        }
    }
    
    pub fn reduce(&self, values: &[Option<bool>]) -> Option<Clause>
    {
        match self
        {
            Clause::C3(l1,l2,l3) => 
            {
                match values[l1.index] {
                    None => {
                        match values[l2.index] {
                            None => match values[l3.index] {
                                None => Some(*self),
                                Some(v) => {
                                    match l3.value 
                                    {
                                        true => if v { None } else { Some(Clause::C2(*l1,*l2)) }
                                        false => if v { Some(Clause::C2(*l1,*l2)) } else { None }    
                                    }
                                }
                            },
                            Some(v) => {
                                match l2.value 
                                {
                                    true => if v { None } else { Clause::C2(*l1,*l3).reduce(values) }
                                    false => if v { Clause::C2(*l1,*l3).reduce(values) } else { None }    
                                }
                            }
                        }
                    },
                    Some(v) => {
                        match l1.value 
                        {
                            true => if v { None } else { Clause::C2(*l2,*l3).reduce(values) }
                            false => if v { Clause::C2(*l2,*l3).reduce(values) } else { None }    
                        }
                    }
                }
            },
            Clause::C2(l1,l2) => 
            {
                match values[l1.index] {
                    None => {
                        match values[l2.index] {
                            None => Some(*self),
                            Some(v) => {
                                match l2.value 
                                {
                                    true => if v { None } else { Some(Clause::C1(*l1)) }
                                    false => if v { Some(Clause::C1(*l1)) } else { None }    
                                }
                            }
                        }
                    },
                    Some(v) => {
                        match l1.value 
                        {
                            true => if v { None } else { Clause::C1(*l2).reduce(values) }
                            false => if v { Clause::C1(*l2).reduce(values) } else { None }    
                        }
                    }
                }
            },
            Clause::C1(l1) => 
            {
                match values[l1.index] {
                    None => Some(*self),
                    Some(v) => {
                        match l1.value 
                        {
                            true => if v { None } else { Some(Clause::Empty) }
                            false => if v { Some(Clause::Empty) } else { None }    
                        }
                    }
                }
            },
            Clause::Empty => 
            {
                return Some(Clause::Empty);
            } 
        }
    }

    pub fn contains(&self, index : usize)-> bool{
        let mut result: bool = false;
        match self
        {
            Clause::C3(l1,l2,l3) => 
            {
                if l1.index == index || l2.index == index || l3.index == index {
                    result = true;
                }
            }, 
            Clause::C2(l1,l2) => 
            {
                if l1.index == index || l2.index == index {
                    result =  true;
                }
            },
            Clause::C1(l1) => 
            {
                if l1.index == index {
                    result = true;
                }
            },
            Clause::Empty => 
            {
                
            }
        }
        result
    }

    pub fn remove(&self, literal: usize)->Clause{
        //return a clause without the literal
        match self
        {
            Clause::C3(l1,l2,l3) => 
            {
                if l1.index == literal {
                    return Clause::C2(*l2,*l3);
                }
                else if l2.index == literal {
                    return Clause::C2(*l1,*l3);
                }
                else if l3.index == literal {
                    return Clause::C2(*l1,*l2);
                }
                else {
                    return Clause::C3(*l1,*l2,*l3);
                }
            }, 
            Clause::C2(l1,l2) => 
            {
                if l1.index == literal {
                    return Clause::C1(*l2);
                }
                else if l2.index == literal {
                    return Clause::C1(*l1);
                }
                else {
                    return Clause::C2(*l1,*l2);
                }
            },
            Clause::C1(l1) => 
            {
                if l1.index == literal {
                    return Clause::Empty;
                }
                else {
                    return Clause::C1(*l1);
                }
            },
            Clause::Empty => 
            {
                return Clause::Empty;
            }
        }
    }

    pub fn invert_literal(&mut self, index : usize){
        match self
        {
            Clause::C3(l1,l2,l3) => 
            {
                if l1.index == index {
                    l1.value = !l1.value;
                }
                else if l2.index == index {
                    l2.value = !l2.value;
                }
                else if l3.index == index {
                    l3.value = !l3.value;
                }
            }, 
            Clause::C2(l1,l2) => 
            {
                if l1.index == index {
                    l1.value = !l1.value;
                }
                else if l2.index == index {
                    l2.value = !l2.value;
                }
            },
            Clause::C1(l1) => 
            {
                if l1.index == index {
                    l1.value = !l1.value;
                }
            },
            Clause::Empty => {}
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn eval()
    {
        let c = Clause::C3(
            Literal{index: 0, value: true, implicated: false}, 
            Literal{index: 1, value: false, implicated: false},
            Literal{index: 2, value: true, implicated: false}
        );

        let mut values = [true, true, false];
        assert_eq!(c.eval(&values), true);
        values = [false, true, true];
        assert_eq!(c.eval(&values), true);
        values = [false, false, false];
        assert_eq!(c.eval(&values), true);
    }

    #[test]
    fn reduce()
    {
        let c = Clause::C3(
            Literal{index: 0, value: true, implicated: false}, 
            Literal{index: 1, value: false, implicated: false}, 
            Literal{index: 2, value: true, implicated: false}
        );

        let mut values: [Option<bool>; 3] = [None, None, None];
        assert_eq!(c.reduce(&values), Some(c));
        values = [ Some(true), None, None];
        assert_eq!(c.reduce(&values), None);
        values = [Some(false), Some(true), Some(false)];
        assert_eq!(c.reduce(&values), Some(Clause::Empty));
    }

    #[test]
    fn clause_invert_literal()
    {
        let mut c3 = Clause::C3(
            Literal{index: 0, value: true, implicated: false}, 
            Literal{index: 1, value: false, implicated: false}, 
            Literal{index: 2, value: true, implicated: false}
        );

        let mut c2 = Clause::C2(
            Literal{index: 0, value: true, implicated: false}, 
            Literal{index: 1, value: false, implicated: false}
        );

        let mut c1 = Clause::C1(
            Literal{index: 0, value: true, implicated: false}
        );

        c3.invert_literal(0);
        c2.invert_literal(0);
        c1.invert_literal(0);

        assert_eq!(c3, Clause::C3(
            Literal{index: 0, value: false, implicated: false}, 
            Literal{index: 1, value: false, implicated: false}, 
            Literal{index: 2, value: true, implicated: false}
        ));

        assert_eq!(c2, Clause::C2(
            Literal{index: 0, value: false, implicated: false}, 
            Literal{index: 1, value: false, implicated: false}
        ));

        assert_eq!(c1, Clause::C1(
            Literal{index: 0, value: false, implicated: false}
        ));

        c3.invert_literal(0);
        assert_eq!(c3, Clause::C3(
            Literal{index: 0, value: true, implicated: false}, 
            Literal{index: 1, value: false, implicated: false}, 
            Literal{index: 2, value: true, implicated: false}
        ));
    }
}
