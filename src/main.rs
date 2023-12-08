use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("resources/1.txt")?;
    let reader = BufReader::new(file);

    let original_map: HashMap<String, i32> = HashMap::from([
        ("one".to_string(), 1), ("two".to_string(), 2), ("three".to_string(), 3), 
        ("four".to_string(), 4), ("five".to_string(), 5),
        ("six".to_string(), 6), ("seven".to_string(), 7), ("eight".to_string(), 8), 
        ("nine".to_string(), 9), ("ten".to_string(), 10),
    ]);

    let mirrored_map: HashMap<String, i32> = original_map
        .iter()
        .map(|(key, &value)| (key.chars().rev().collect::<String>(), value))
        .collect();

    let res: i32 = reader
        .lines()
        .filter_map(|line_result| line_result.ok())
        .map(|line| {
            let first = first_digit_or_string_digit(&line, &original_map);
            let last = first_digit_or_string_digit(&line.chars().rev().collect::<String>(), &mirrored_map);
            // Creaate 2 digit number
            first * 10 + last
        })
        .sum();

    println!("Calibration value: {}", res);

    Ok(())
}

fn first_digit_or_string_digit(input: &str, hashmap: &HashMap<String, i32>) -> i32 {
    let mut first_occurrence = (input.len(), None);

    for (key, &value) in hashmap {
        if let Some(index) = input.find(key) {
            if index < first_occurrence.0 {
                first_occurrence = (index, Some(value));
            }
        }
    }

    if let Some(index) = input.chars().position(|c| c.is_digit(10)) {
        if index < first_occurrence.0 {
            first_occurrence = (index, input.chars().nth(index).and_then(|c| c.to_digit(10)).map(|d| d as i32));
        }
    }

    first_occurrence.1.unwrap_or(0)
}

