use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::collections::HashSet;

fn part1(path: &String) -> Result<(), Error> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut total = 0;
    let mut answers = HashSet::new();

    for line in buffered.lines()
    {
        let line_str = line.unwrap();

        if line_str.len() == 0
        {
            // record; next group
            total += answers.len();
 
            answers.clear();
        }
        else
        {
            for ch in line_str.chars()
            {
                answers.insert(ch);

            }
        }
    }
    // last group
    total += answers.len();
 
    println!("{}", total);

    Ok(())
}

fn part2(path: &String) -> Result<(), Error> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut total = 0;
    let mut answers = HashSet::new();
    let mut first_in_group = true;

    for line in buffered.lines()
    {
        let line_str = line.unwrap();

        if line_str.len() == 0
        {
            // record; next group
            total += answers.len();
 
            answers.clear();
            first_in_group = true;
        }
        else
        {
            let mut my_answers = HashSet::new();
            for ch in line_str.chars()
            {
                my_answers.insert(ch);
            }

            if first_in_group
            {
                answers = my_answers;
                first_in_group = false;
            }
            else
            {
                answers = answers.intersection(&my_answers).cloned().collect();
            }
        }
    }
    // last group
    total += answers.len();
    println!("{}", total);

    Ok(())
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1]; // "sample.txt";

    return part2(path);
}
