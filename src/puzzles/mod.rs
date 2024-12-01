use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

pub mod day_01;

pub fn run_day(day: &str) -> Result<String, Box<dyn Error>> {
    match day {
        "1a" => run_puzzle(day_01::solve_a, "day_01_a.txt"),
        "1b" => run_puzzle(day_01::solve_b, "day_01_a.txt"),
        _ => Err(format!("No solution implemented for day {}", day).into()),
    }
}

fn run_puzzle(
    solve: fn(Lines<BufReader<File>>) -> Option<String>,
    input_file: &str,
) -> Result<String, Box<dyn Error>> {
    let input_path = format!("src/inputs/{}", input_file);
    let file: File = File::open(&input_path)
        .map_err(|e| format!("Error opening file '{}': {}", input_path, e))?;

    let lines = BufReader::new(file).lines();

    solve(lines).ok_or_else(|| "No result returned by the solver".into())
}
