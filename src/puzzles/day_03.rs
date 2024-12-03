use regex::Regex;
use std::fs::File;
use std::io::{BufReader, Lines};

pub fn solve_a(lines: Lines<BufReader<File>>) -> Option<String> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mults: i32 = lines
        .filter_map(Result::ok)
        .map(|line| {
            re.captures_iter(&line)
                .map(|caps| {
                    let a: i32 = caps[1].parse().unwrap();
                    let b: i32 = caps[2].parse().unwrap();
                    a * b
                })
                .sum::<i32>()
        })
        .sum();

    return Some(mults.to_string());
}

pub fn solve_b(lines: Lines<BufReader<File>>) -> Option<String> {
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let input: String = lines.filter_map(Result::ok).collect::<Vec<_>>().join("\n");

    let mut out = 0;
    let mut enabled = true;

    for cap in re.captures_iter(&input) {
        let str = &cap[0];
        match &str[..3] {
            "mul" => {
                if enabled {
                    let nums: Vec<i32> = str[4..str.len() - 1]
                        .split(',')
                        .map(|s| s.parse().unwrap())
                        .collect();
                    out += nums[0] * nums[1];
                }
            }
            "do(" => {
                enabled = true;
            }
            "don" => {
                enabled = false;
            }
            _ => panic!("Invalid instruction"),
        }
    }

    return Some(out.to_string());
}
