pub mod puzzles;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // No arguments passed
        1 => {
            println!("Running all puzzle solutions:");
            for day in 1..=25 {
                match puzzles::run_day(format!("{}a", day).as_str()) {
                    Ok(result) => println!("Day {}a: {}", day, result),
                    Err(e) => eprintln!("Day {}a: {}", day, e),
                }
                match puzzles::run_day(format!("{}b", day).as_str()) {
                    Ok(result) => println!("Day {}b: {}", day, result),
                    Err(e) => eprintln!("Day {}b: {}", day, e),
                }
            }
        }
        // One argument passed
        2 => {
            let mut days: Vec<String> = Vec::new();

            if args[1].len() == 1 {
                days.push(format!("{}a", args[1]));
                days.push(format!("{}b", args[1]));
            } else {
                days.push(args[1].to_string());
            };

            for day in days {
                match puzzles::run_day(&day) {
                    Ok(result) => println!("Day {}: {}", day, result),
                    Err(err) => eprintln!("Day {}: {}", day, err),
                }
            }
        }
        _ => {
            eprintln!("Usage:");
            eprintln!("  advent_of_code            # Run all puzzles");
            eprintln!("  advent_of_code <day>      # Run specific day");
        }
    }
}
