use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut value = 0;
    let mut value2 = 0;
    if let Ok(lines) = read_lines(&args[1]) {
        for line in lines {
            value += match line {
                Ok(ref line) => {
                    match &line as &str {
                        "A X" => 4, // Rock v Rock
                        "A Y" => 8, // Rock v Paper
                        "A Z" => 3, // Rock v Scissors
                        "B X" => 1, // Paper v Rock
                        "B Y" => 5, // Paper v Paper
                        "B Z" => 9, // Paper v Scissors
                        "C X" => 7, // Scissors v Rock
                        "C Y" => 2, // Scissors v Paper
                        "C Z" => 6, // Scissors v Scissors
                        &_ => 0,
                    }
                },
                Err(error) => panic!("Problem opening file or reading lines: {:?}", error),
            };
            value2 += match line {
                Ok(ref line) => {
                    match &line as &str {
                        "A X" => 3, // Rock, Loss     => Scissors
                        "A Y" => 4, // Rock, Draw     => Rock
                        "A Z" => 8, // Rock, Win      => Paper
                        "B X" => 1, // Paper, Loss    => Rock
                        "B Y" => 5, // Paper, Draw    => Paper
                        "B Z" => 9, // Paper, Win     => Scissors
                        "C X" => 2, // Scissors, Loss => Paper
                        "C Y" => 6, // Scissors, Draw => Scissors
                        "C Z" => 7, // Scissors, Win  => Rock
                        &_ => 0,
                    }
                },
                Err(error) => panic!("Problem opening file or reading lines: {:?}", error),
            };
        }
    }
    println!("First Part: {}, Second Part: {}", value, value2)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

