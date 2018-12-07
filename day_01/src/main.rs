use std::{
    str::FromStr,
    io::BufRead
};

fn get_increments(input_file: &str) -> Result<std::vec::Vec<i32>, std::io::Error> {
    let input = match std::fs::File::open(input_file) {
        Ok(v) => v,
        Err(e) => return Err(e)
    };

    let buffered = std::io::BufReader::new(input);

    let mut increments: std::vec::Vec<i32> = std::vec::Vec::new();

    for line in buffered.lines() {
        let line_str = &line.unwrap();
        let increment = i32::from_str(line_str).unwrap();
        increments.push(increment);
    }

    Ok(increments)
}

fn get_frequency(increments: &std::vec::Vec<i32>) -> i32 {
    increments.into_iter().sum::<i32>()
}

fn get_repeated_frequency(increments: &std::vec::Vec<i32>) -> i32 {
    
    let mut frequencies: std::vec::Vec<i32> = vec![0];
    let mut frequency: i32 = 0;
    
    'outer: loop {
        for increment in increments {
            frequency += increment;

            if frequencies.contains(&frequency) {
                break 'outer;
            }
            else {
                frequencies.push(frequency);
            }        
        }
    }
   
    frequency
}

fn main() {
    let args: std::vec::Vec<String> = std::env::args().collect();

    match args.len() {
        1 => {
            println!("Please supply input file as parameter");
        },

        2 => {
            let input_file = &args[1];
            let increments = match get_increments(input_file) {
                Ok(v) => v,
                Err(error) => {
                                println!("Error found while retrieving increments from {}:\n{}", input_file, error);
                                return
                            }
            };

            let frequency = get_frequency(&increments);
            println!("The resulting frequency is {}", frequency);

            let repeated_frequency = get_repeated_frequency(&increments);
            println!("The first frequency my device reaches twice is {}", repeated_frequency);            
        },

        _ => {
            println!("Please supply just 1 parameter: input file");
        }
    }
}
