use std::env;
use dimacs::parse_dimacs;

pub fn main()-> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    println!("Path given: {:?}", args);
    let path = &args[1];
    println!("Path given: {:?}", path);

    //parse the file at path
    let sat_instance= parse_dimacs(path).unwrap();

    println!("SAT instance: {:?}", sat_instance);
    
    Ok(())
}


