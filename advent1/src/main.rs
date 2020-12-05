use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use itertools::Itertools;

fn main() -> Result<(), Error> {
    let path = "test.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let v: Vec<i32> = buffered.lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();

    for result in v
        .into_iter()
        .combinations(2)
        .filter(|pair| pair.first().unwrap() + pair.last().unwrap() == 2020)
        .map(|pair| pair.first().unwrap() * pair.last().unwrap())
    {
        println!("{}", result);
    }

    Ok(())
}
