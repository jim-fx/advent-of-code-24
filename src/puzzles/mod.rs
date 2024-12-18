use std::{
    error::Error,
    fmt,
    fs::{self, File},
    io::{BufRead, BufReader, Lines, Write},
    path::Path,
    time::{Duration, Instant},
};

use reqwest::blocking::Client;

pub mod day_01;
pub mod day_02;
pub mod day_03;

fn human_readable_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let total_nanos = duration.subsec_nanos();

    let days = total_seconds / 86400;
    let hours = (total_seconds % 86400) / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    // Breakdown of nanoseconds into larger units
    let milliseconds = total_nanos / 1_000_000;
    let microseconds = (total_nanos % 1_000_000) / 1_000;
    let nanoseconds = total_nanos % 1_000;

    let mut parts = Vec::new();

    if days > 0 {
        parts.push(format!("{}d", days));
    }
    if hours > 0 {
        parts.push(format!("{}h", hours));
        if parts.len() > 1 {
            return parts.join("");
        }
    }
    if minutes > 0 {
        parts.push(format!("{}min", minutes));
        if parts.len() > 1 {
            return parts.join("");
        }
    }
    if seconds > 0 {
        parts.push(format!("{}s", seconds));
        if parts.len() > 1 {
            return parts.join("");
        }
    }
    if milliseconds > 0 {
        parts.push(format!("{}ms", milliseconds));
        if parts.len() > 1 {
            return parts.join("");
        }
    }
    if microseconds > 0 {
        parts.push(format!("{}µs", microseconds));
        if parts.len() > 1 {
            return parts.join("");
        }
    }
    // Include nanoseconds only if there's a non-zero remainder or if nothing else was added
    if nanoseconds > 0 || parts.is_empty() {
        parts.push(format!("{}ns", nanoseconds));
        if parts.len() > 1 {
            return parts.join("");
        }
    }

    parts.join("")
}

pub struct PuzzleResult {
    pub duration: Duration,
    pub answer: String,
}
impl fmt::Display for PuzzleResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} in {}",
            self.answer,
            human_readable_duration(self.duration)
        )
    }
}

pub fn run_day(day: &str, test: bool) -> Result<PuzzleResult, Box<dyn Error>> {
    match day {
        "1a" => run_puzzle(day_01::solve_a, "day_01.txt", test),
        "1b" => run_puzzle(day_01::solve_b, "day_01.txt", test),
        "2a" => run_puzzle(day_02::solve_a, "day_02.txt", test),
        "2b" => run_puzzle(day_02::solve_b, "day_02.txt", test),
        "3a" => run_puzzle(day_03::solve_a, "day_03.txt", test),
        "3b" => run_puzzle(day_03::solve_b, "day_03.txt", test),
        _ => Err(format!("No solution implemented for day {}", day).into()),
    }
}

fn run_puzzle(
    solve: fn(Lines<BufReader<File>>) -> Option<String>,
    input_file: &str,
    test: bool,
) -> Result<PuzzleResult, Box<dyn Error>> {
    let mut input_path = format!("src/inputs/{}", input_file);
    if test {
        input_path = input_path.replace(".txt", "_test.txt");
    }
    let file: File = File::open(&input_path)
        .map_err(|e| format!("Error opening file '{}': {}", input_path, e))?;

    let lines = BufReader::new(file).lines();

    let start = Instant::now();
    let answer: String = solve(lines).ok_or_else(|| "No result returned by the solver")?;
    let duration = start.elapsed();

    Ok(PuzzleResult { duration, answer })
}

pub fn download_input_file(day: u8) -> Result<(), Box<dyn Error>> {
    let token = std::fs::read_to_string(".token")?.trim().to_string();
    let url = format!("https://adventofcode.com/2024/day/{}/input", day);
    let client = Client::new();
    let response = client
        .get(url)
        .header("Cookie", format!("session={}", token))
        .send()?;
    if !response.status().is_success() {
        return Err(format!("Request failed with status code {}", response.status()).into());
    }
    let content = response.text()?;
    fs::create_dir_all("src/inputs")?;

    let file_path = Path::new("src/inputs").join(format!("day_{:02}.txt", day));
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;

    println!(
        "Downloaded src/inputs/day_{:02}.txt with {} lines",
        day,
        content.lines().count()
    );

    Ok(())
}
