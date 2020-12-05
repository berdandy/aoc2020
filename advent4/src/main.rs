use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let path = &args[1]; // "sample.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut lines: Vec<_> = buffered.lines().map(|line| { line.unwrap() }).collect();
    lines.reverse();

    let required_fids = [ "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" ];

    let mut valid_passports = 0;
    let mut required_count = 0;
    for line in lines
    {
        // split line into fid:value pairs
        let fields: Vec<_> = line.split_whitespace().collect();
        for field in fields.iter()
        {
            let pair: Vec<_> = field.split(':').collect();
            let fid = pair.first().unwrap();
            let value = pair.last().unwrap();
            if required_fids.contains(pair.first().unwrap())
            {
                let valid : bool = match fid.as_ref() {
                    "byr" => (value.parse::<i32>().unwrap() >= 1920 && value.parse::<i32>().unwrap() <= 2002),
                    "iyr" => (value.parse::<i32>().unwrap() >= 2010 && value.parse::<i32>().unwrap() <= 2020),
                    "eyr" => (value.parse::<i32>().unwrap() >= 2020 && value.parse::<i32>().unwrap() <= 2030),
                    "hgt" => (value.len() == 5 && value.ends_with("cm") && value.get(..3).unwrap().parse::<i32>().unwrap() >= 150 && value.get(..3).unwrap().parse::<i32>().unwrap() <= 193) ||
                             (value.len() == 4 && value.ends_with("in") && value.get(..2).unwrap().parse::<i32>().unwrap() >= 59 && value.get(..2).unwrap().parse::<i32>().unwrap() <= 76),
                    "hcl" => value.len() == 7 && value.starts_with("#") && isize::from_str_radix(value.get(1..).unwrap(), 16).is_ok(),
                    "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(value),
                    "pid" => value.len() == 9 && value.chars().all(char::is_numeric),
                    _ => false
                };

                if valid
                {
                    required_count += 1;
                }
                // println!("{} valid:{}", field, valid);

            }
        }

        if line.len() == 0
        {
            // println!("---");
            // end of record, store and prep for next
            if required_count == required_fids.len()
            {
                valid_passports += 1;
            }

            required_count = 0;
        }
    }
    if required_count == required_fids.len()
    {
        valid_passports += 1;
    }

    // println!("---");
    println!("{}", valid_passports);
    Ok(())
}
