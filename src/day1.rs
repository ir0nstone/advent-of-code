use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn calibration_value(string: String) -> Result<i8, &'static str> {
    // create a dictionary of the word number mapping to the character
    let mut str_nums: HashMap<String, char> = HashMap::new();

    str_nums.insert("zero".to_string(), '0');
    str_nums.insert("one".to_string(), '1');
    str_nums.insert("two".to_string(), '2');
    str_nums.insert("three".to_string(), '3');
    str_nums.insert("four".to_string(), '4');
    str_nums.insert("five".to_string(), '5');
    str_nums.insert("six".to_string(), '6');
    str_nums.insert("seven".to_string(), '7');
    str_nums.insert("eight".to_string(), '8');
    str_nums.insert("nine".to_string(), '9');

    // store the first and last characters
    let mut first_char = '_';
    let mut last_char = '_';

    // loop through all characters and their index
    for (idx, mut c) in string.char_indices() {
        // iterate through all word forms, checking if the current string begins with it
        for k in str_nums.keys() {
            if string[idx..].starts_with(k) {
                // if so, set the current char to actually be the number character
                c = str_nums[k]
            }
        }

        // if not a digit, continue looping
        if !c.is_digit(10) {
            continue;
        }

        // if first_char has not been set yet, we are the the first character, overwrite it
        if !first_char.is_digit(10) {
            first_char = c;
        }

        last_char = c;
    }

    let mut digit_string = String::new();
    digit_string.push(first_char);
    digit_string.push(last_char);

    match digit_string.parse::<i8>() {
        Ok(parsed) => return Ok(parsed),
        Err(_) => return Err("Failed to parse the string as an integer")
    }
}

fn calculate_all_calibration_values()  -> Vec<i8> {
    let mut calibration_values: Vec<i8> = Vec::new();

    let f = File::open("src/inputs/day1").expect("Unable to open file");
    let reader = BufReader::new(f);

    for l in reader.lines() {
        let line = l.unwrap();

        match calibration_value(line) {
            Ok(parsed) => calibration_values.push(parsed),
            Err(_) => break
        }
    }

    return calibration_values;
}

pub fn do_task() {
    let calibration_values = calculate_all_calibration_values();

    let mut sum: i64 = 0;

    for v in calibration_values {
        sum += v as i64;
    }

    println!("{}", sum);
}
