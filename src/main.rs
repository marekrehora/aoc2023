use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("/Users/rehora/DEV/rust/aoc2023/resources/1.txt")?;
    let reader = BufReader::new(file);

    let _original_map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("ten", 10),
    ]);

    let mirrored_map: HashMap<String, i32> = _original_map
        .iter()
        .map(|(key, &value)| (key.chars().rev().collect::<String>(), value))
        .collect();

    let original_map: HashMap<String, i32> = _original_map
        .iter()
        .map(|(&key, &value)| (key.to_string(), value))
        .collect();

    let res: Result<i32, _> = reader
        .lines()
        .map(|line_result| {
            line_result.and_then(|line| {
                let first = first_digit_or_string_digit(&line, &original_map);
                let last =
                    first_digit_or_string_digit(&line.chars().rev().collect::<String>(), &mirrored_map);
                Ok(first * 10 + last)
            })
        })
        .sum();

    match res {
        Ok(value) => {
            println!("Calibration value: {}", value);
        }
        Err(e) => eprintln!("Error processing file: {}", e),
    }

    Ok(())
}

fn first_digit_or_string_digit(input: &str, hashmap: &HashMap<String, i32>) -> i32 {
    let mut first_occurrence = (input.len(), None);

    // Check for each word in the map
    for (key, &value) in hashmap {
        if let Some(index) = input.find(key) {
            if index < first_occurrence.0 {
                first_occurrence = (index, Some(value));
            }
        }
    }

    // Check for the first digit
    if let Some(index) = input.chars().position(|c| c.is_digit(10)) {
        if index < first_occurrence.0 {
            // Convert the found digit to i32
            if let Some(digit) = input.chars().nth(index).and_then(|c| c.to_digit(10)) {
                first_occurrence = (index, Some(digit as i32));
            }
        }
    }

    // Check if any occurrence was found
    match first_occurrence {
        (_, Some(value)) => value,
        _ => 0,
    }
}
