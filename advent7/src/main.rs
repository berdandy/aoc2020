use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::collections::HashSet;

fn part1(path: &String) -> Result<(), Error> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let lines: Vec<String> = buffered.lines().collect::<Result<_, _>>().unwrap();

    let mut bags: HashSet<String> = HashSet::new();
    let mut targets: HashSet<String>  = HashSet::new();
    targets.insert("shiny gold".to_string());
    
    loop
    {
        let mut new_targets: HashSet<String>  = HashSet::new();
        for target in targets.drain()
        {
            for line in &lines
            {
                if line.contains(&target) && !line.starts_with(&target)
                {
                    let bagname  = line.split(" bags").next().unwrap().to_string();
                    bags.insert(bagname.clone());
                    new_targets.insert(bagname.clone());
                }
            }
        }

        if new_targets.is_empty()
        {
            break;
        }
        else
        {
            targets = new_targets;
        }
    }

    /*
    for bag in bags.iter()
    {
        print!("{}, ", bag);
    }
    */
    println!("{}", bags.len());

    Ok(())
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1]; // "sample.txt";

    return part1(path);
}
