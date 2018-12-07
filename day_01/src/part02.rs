pub fn get_repeated_frequency(increments: &std::vec::Vec<i32>) -> i32 {
    
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
