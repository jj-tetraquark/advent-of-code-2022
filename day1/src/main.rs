use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Need to provide an input file as a second argument. \
                               Number of arguments is not 2");
    
    let mut sacks = fs::read_to_string(&args[1])
                    .unwrap()
                    .lines()
                    .fold(vec![0], | mut acc, line | {
                        match line.parse::<u32>() {
                            Err(_) => {
                                acc.push(0);
                                return acc
                            }
                            Ok(calories) => {
                                *acc.last_mut().unwrap() += calories;
                                return acc
                            }
                        }
                    });
    sacks.sort();

    println!("Max cals: {}", sacks.last().unwrap());
    println!("Top 3 sum: {}", sacks.iter().rev().take(3).sum::<u32>())
}

