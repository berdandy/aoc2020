use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "test.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let right = 3;
    let down = 1;

    let mut trees_hit = 0;

    for (i, line) in buffered.lines().step_by(down).enumerate()
    {
        let line_data: String = line.unwrap();
        if line_data.chars().nth((i * right) % line_data.len()).unwrap() == '#'
        {
            trees_hit += 1;
        }
    }
    println!("{}", trees_hit);

    Ok(())
}
