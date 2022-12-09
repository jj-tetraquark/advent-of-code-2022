use std::fs;
use std::env;
use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0
        }
    }
}

fn simulate_rope_dynamics(input: &str, rope_len: usize) -> usize {
    let mut rope = vec![Coord::new(); rope_len];
    let mut visited: HashSet<Coord> = HashSet::new();

    input.lines().for_each(|line| {
        let parts: Vec<&str> = line.split(' ').collect();
        //println!("{:?}", parts);
        for _ in 0..parts[1].parse::<usize>().unwrap() {

            match parts[0] {
                "L" => rope[0].x -= 1,
                "R" => rope[0].x += 1,
                "U" => rope[0].y += 1,
                "D" => rope[0].y -= 1,
                _ => panic!()
            }

            for i in 1..rope.len() {
                let diff_x = rope[i-1].x - rope[i].x;
                let diff_y = rope[i-1].y - rope[i].y;
            
                if diff_x.abs() > 1 || diff_y.abs() > 1 {
                    rope[i].x += diff_x.signum();
                    rope[i].y += diff_y.signum();
                }
            }
            visited.insert(rope.last().unwrap().clone());
        }
    });
    visited.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Need to provide an input file as a second argument. \
                               Number of arguments is not 2");
    
    
    let input = fs::read_to_string(&args[1]).unwrap();

    println!("Tail visited {} coords", simulate_rope_dynamics(&input, 2));
    println!("Tail visited {} coords", simulate_rope_dynamics(&input, 10));
}
