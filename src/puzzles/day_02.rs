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

    let mut total_valid = 0;

    for row in rows {
        let is_asc = row[0] < row[1];
        let mut is_valid = true;

        for pair in row.windows(2) {
            let diff = (pair[0] - pair[1]).abs();

            if (is_asc && pair[0] > pair[1])
                || (!is_asc && pair[0] < pair[1])
                || diff < 1
                || diff > 3
            {
                is_valid = false;
                break;
            }
        }

        if is_valid {
            total_valid += 1;
        }
    }

    return Some(total_valid.to_string());
}

fn is_valid_row(row: Vec<i32>) -> bool {
    let is_asc = row[0] < row[1];
    for pair in row.windows(2) {
        let diff = (pair[0] - pair[1]).abs();
        if (is_asc && pair[0] > pair[1]) || (!is_asc && pair[0] < pair[1]) || diff < 1 || diff > 3 {
            return false;
        }
    }
    return true;
}

pub fn solve_b(lines: Lines<BufReader<File>>) -> Option<String> {
    let rows = lines
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
        .into_iter();

    let mut total_valid = 0;

    for row in rows {
        if is_valid_row(row.clone()) {
            total_valid += 1;
            continue;
        }
        for i in 0..row.len() {
            let mut new_row = row.clone();
            new_row.remove(i);
            if is_valid_row(new_row) {
                total_valid += 1;
                break;
            }
        }
    }

    return Some(total_valid.to_string());
}
