pub mod puzzles;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // No arguments passed
        1 => {
            println!("Running all puzzle solutions:");
            for day in 1..=25 {
                match puzzles::run_day(format!("{}a", day).as_str(), false) {
                    Ok(result) => println!("Day {}a: {}", day, result),
                    Err(e) => eprintln!("Day {}a: {}", day, e),
                }
                match puzzles::run_day(format!("{}b", day).as_str(), false) {
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
                match puzzles::run_day(&day, false) {
                    Ok(result) => println!("Day {}: {}", day, result),
                    Err(err) => eprintln!("Day {}: {}", day, err),
                }
            }
        }
        3 => match args[1].as_str() {
            "download" => {
                let day = args[2].parse::<u8>().unwrap();
                puzzles::download_input_file(day).unwrap();
            }
            _ => {
                if args[1].len() == 1 {
                    let mut days: Vec<String> = Vec::new();
                    days.push(format!("{}a", args[1]));
                    days.push(format!("{}b", args[1]));
                    for day in days {
                        match puzzles::run_day(&day, false) {
                            Ok(result) => println!("Day {}: {}", day, result),
                            Err(err) => eprintln!("Day {}: {}", day, err),
                        }
                    }
                } else if args[1].len() == 2 {
                    let test = args[2].eq("--test");

                    let test_str = if test { " (test)" } else { "" };

                    match puzzles::run_day(&args[1], test) {
                        Ok(result) => println!("Day {}{}: {}", args[1], test_str, result),
                        Err(err) => eprintln!("Day {}{}: {}", args[1], test_str, err),
                    }
                }
            }
        },
        _ => {
            eprintln!("Usage:");
            eprintln!("  advent_of_code            # Run all puzzles");
            eprintln!("  advent_of_code <day>      # Run specific day");
        }
    }
}
