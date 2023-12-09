use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub fn main() -> Result<()> {
    let file = File::open("resources/2.txt")?;
    let reader = BufReader::new(file);

    let (total, total_color_power) = reader
        .lines()
        .filter_map(|line_result| line_result.ok())
        .map(|line| validate_game(&line))
        .fold((0, 0), |(total_ids, cube_power), (game_id, power)| {
            (total_ids + game_id, cube_power + power)
        });

    println!("Total valid game Ids: {}", total);
    println!("Least colors multiplied: {}", total_color_power);

    Ok(())
}

fn validate_game(input: &str) -> (i32, i32) {
    let limits: HashMap<String, i32> = HashMap::from([
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]);

    let game_regex = Regex::new(r"Game (\d+):").unwrap();
    let game_number = game_regex
        .captures(input)
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse::<i32>().ok())
        .expect("Failed to parse game number");

    // Split the string after "Game x: "
    let parts: Vec<&str> = game_regex.split(input).skip(1).collect();

    let mut max_counts: HashMap<String, i32> = HashMap::new();

    for part in parts {
        let part_max_counts = find_max_counts_per_color(part);
        for (color, count) in part_max_counts {
            max_counts
                .entry(color)
                .and_modify(|e| *e = (*e).max(count))
                .or_insert(count);
        }
    }

    let is_within_limits = check_limits(&max_counts, &limits);

    let sum_color_power = max_counts.iter().map(|(_, &count)| count).product::<i32>();

    if is_within_limits {
        (game_number, sum_color_power)
    } else {
        (0, sum_color_power)
    }
}

fn find_max_counts_per_color(input: &str) -> HashMap<String, i32> {
    let color_regex = Regex::new(r"(\d+) (\w+)").unwrap();
    let mut max_counts = HashMap::new();

    for cap in color_regex.captures_iter(input) {
        let count = cap[1].parse::<i32>().unwrap();
        let color = cap[2].to_string();

        max_counts
            .entry(color)
            .and_modify(|e: &mut i32| *e = (*e).max(count))
            .or_insert(count);
    }

    max_counts
}

fn check_limits(counts: &HashMap<String, i32>, limits: &HashMap<String, i32>) -> bool {
    for (color, &count) in counts {
        if let Some(&limit) = limits.get(color) {
            if count > limit {
                return false;
            }
        }
    }
    true
}
