use std::{
    env,
    str::FromStr,
    fs::File,
    vec::Vec,
    io::BufRead
};

fn part_01(input_file: &str) -> Result<i32, std::io::Error> {
    let input = match File::open(input_file) {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    let buffered = std::io::BufReader::new(input);

    let mut result: i32 = 0;

    for line in buffered.lines() {
        let line_str = &line.unwrap();
        result += i32::from_str(line_str).unwrap();
    }

    Ok(result)
}

fn part_02(input_file: &str) -> Result<i32, std::io::Error> {
    let input = match File::open(input_file) {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    let buffered = std::io::BufReader::new(input);

    let mut increments: Vec<i32> = Vec::new();
    let mut frequencies: Vec<i32> = vec![0];

    for line in buffered.lines() {
        let line_str = &line.unwrap();
        let increment = i32::from_str(line_str).unwrap();
        increments.push(increment);
    }

    let mut frequency: i32 = 0;
    
    'outer: loop {
        for increment in &increments {
            frequency += increment;

            if frequencies.contains(&frequency) {
                break 'outer;
            }
            else {
                frequencies.push(frequency);
            }        
        }
    }
   
    Ok(frequency)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            println!("Please supply input file as parameter");
        },
        2 => {
            let input_file = &args[1];

            println!("{}", "-".repeat(79));
            match part_01(input_file) {
                Ok(result) => println!("The resulting frequency is {}", result),
                Err(error) => { println!("Error found while calculating solution for Part 1: {}", error)}
            };
            println!("{}", "-".repeat(79));

            println!("{}", "-".repeat(79));
            match part_02(input_file) {
                Ok(frequency) => println!("The first frequency my device reaches twice is {}", frequency),
                Err(error) => { println!("Error found while calculating solution for Part 2: {}", error)}
            };
            println!("{}", "-".repeat(79));            
        },
        _ => {
            println!("Please supply just 1 parameter: input file");
        }
    }
}
