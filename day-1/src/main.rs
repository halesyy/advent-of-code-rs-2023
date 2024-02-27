/*
 * Goal: 
 * - Read input.txt, line by line
 * - Index the numbers in the sequence 
 * - Product 1st/Last (0, -1 index)
 * - Combine into a num e.g. f"{a}{b}"
 * - Convert to int
 * - Sum all numbers
 * - That is output
 */

use std::fs::read_to_string;

const INPUT_FILE: &str = "input.txt";

fn consume_parse_into_line_value(line: &str) -> u32 {
    let digits: Vec<u32> = line
        .chars()
        .filter_map(|c| {
            if c.is_numeric() {
                // Parse the char -> string -> parse into u32 from str.
                let parsed_u32: u32 = String::from(c).parse()
                    .expect(format!("Could not parse {} into a string/u32", c).as_str());
                Some(parsed_u32)
            } else {
                None
            }
        })
        .collect();

    // Calculate the values based on first/last of the array.
    let a = digits.first().unwrap();
    let b = digits.last().unwrap();
    let interp = format!("{}{}", a, b);
    let interp_num: u32 = interp.parse().unwrap();
    interp_num
}

fn main() {
    
    let file_data = read_to_string(INPUT_FILE)
        .expect("Could not read the file: input.txt");
    let lines = file_data.split("\n");

    // Iter over each line.
    let mut running_sum: u32 = 0;

    for line in lines.into_iter() {
        let val = consume_parse_into_line_value(line);
        println!("{} = {}", line, val);
        running_sum += val;
    }

    println!("{}", running_sum);
}
