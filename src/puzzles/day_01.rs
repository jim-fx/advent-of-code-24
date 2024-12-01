use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Lines};

pub fn solve_a(lines: Lines<BufReader<File>>) -> Option<String> {
    let rows = lines
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
        .into_iter();

    let mut first_column = rows.clone().map(|row| row[0]).collect::<Vec<i32>>();
    first_column.sort_unstable();

    let mut second_column = rows.map(|row| row[1]).collect::<Vec<i32>>();
    second_column.sort_unstable();

    let total_distance: i32 = first_column
        .iter()
        .zip(second_column.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    Some(total_distance.to_string())
}

pub fn solve_b(lines: Lines<BufReader<File>>) -> Option<String> {
    let rows = lines
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut second_column_map: HashMap<i32, i32> = HashMap::new();
    for row in &rows {
        *second_column_map.entry(row[1]).or_insert(0) += 1;
    }

    let total_distance: i32 = rows
        .iter()
        .map(|row| row[0] * *second_column_map.get(&row[0]).unwrap_or(&0))
        .sum();

    Some(total_distance.to_string())
}
