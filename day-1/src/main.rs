/*
 * Goal: 
 * - Read input.txt, line by line
 * - Index the numbers in the sequence 
 * - Product 1st/Last (0, -1 index)
 * - Combine into a num e.g. f"{a}{b}"
 * - Convert to int
 * - Sum all numbers
 * - That is output
 * 
 * Part 2:
 * - The digits are encoded sometimes as strings, valid
 *   digits are:
 *   - one, two, three, four, five, six, seven, eight, nine
 * - Use this info to parse the correct numbers 
 */

#![allow(dead_code, unused_variables)]

use std::fs::read_to_string;
use aho_corasick::AhoCorasick;

const INPUT_FILE: &str = "input.txt";

// fn consume_parse_into_line_value(line: &str) -> u32 {
//     let digits: Vec<u32> = line
//         .chars()
//         .filter_map(|c| {
//             if c.is_numeric() {
//                 // Parse the char -> string -> parse into u32 from str.
//                 let parsed_u32: u32 = String::from(c).parse()
//                     .expect(format!("Could not parse {} into a string/u32", c).as_str());
//                 Some(parsed_u32)
//             } else {
//                 None
//             }
//         })
//         .collect();
//     // Calculate the values based on first/last of the array.
//     let a = digits.first().unwrap();
//     let b = digits.last().unwrap();
//     let interp = format!("{}{}", a, b);
//     let interp_num: u32 = interp.parse().unwrap();
//     interp_num
// }

const DIGIT_PATTERNS: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    "1", "2", "3", "4", "5", "6", "7", "8", "9"
];

const DIGIT_REPLACEMENTS: [&str; 9] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9"
];

fn line_to_combined_digit(line: &str) -> u32 {
    let ah = AhoCorasick::new(&DIGIT_PATTERNS).unwrap();
    let overlapping = ah.find_overlapping_iter(line);

    // Store all digits, i.e. 1, 2, 5 etc.
    let mut string_digits: Vec<String> = Vec::new();
    for digit_word_match in overlapping {
        // 0 = "one", 1 = "two", etc. so last digit char is for 
        // specifically 8, 9+ are direct matches.
        let pattern_id = digit_word_match.pattern().as_usize();
        // Get the string.
        let digit_string = match pattern_id {
            // 1 .. 8
            p if p <= 8 => String::from(DIGIT_REPLACEMENTS[pattern_id]),

            // 9+ .. onwards
            p if p >= 9 => format!("{}", (p - 9 + 1)),

            // Catch for safety.
            _ => panic!("Failed to match to the expected circumstances")
        };
        string_digits.push(digit_string);
    }

    println!("{} -> {:?}", line, string_digits);

    let a = string_digits.first().unwrap();
    let b = string_digits.last().unwrap();
    let combined_ab = format!("{}{}", a, b);

    combined_ab.parse::<u32>().unwrap()
}

fn main() {
    let file_data = read_to_string(INPUT_FILE)
        .expect("Could not read the file: input.txt");
    let lines = file_data.split("\n");

    // Iter over each line.
    let mut running_sum: u32 = 0;

    for line in lines.into_iter() {
        let val = line_to_combined_digit(line);

        // continue;

        // println!("{line}\n->\n{val}\n");
        // let fl_value = consume_parse_into_line_value(&repline);

        // let replaced_line = string_digit_replace(line);
        // let val = consume_parse_into_line_value(&replaced_line);
        // println!("{} = {} = {}", line, replaced_line, val);
        running_sum += val;
    }

    println!("{}", running_sum);
}
