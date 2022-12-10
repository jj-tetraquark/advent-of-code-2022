use std::collections::VecDeque;
use std::env;
use std::fs;

#[derive(Debug)]
struct State {
    x: i64,
    add_x_buffer: VecDeque<i64>,
}

impl State {
    fn new() -> Self {
        Self {
            x: 1,
            add_x_buffer: VecDeque::new(),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Need to provide an input file as a second \
                               argument. Number of arguments is not 2");

    let input = fs::read_to_string(&args[1]).unwrap();

    let mut state = State::new();
    let mut cycle: i64 = 0;
    let mut lines = input.lines().peekable();
    let mut signal_strengths: Vec<i64> = Vec::new();

    let mut CRT: Vec<Vec<char>> = vec![vec!['.'; 40]; 6];

    loop {
        let row = (cycle / 40) as usize;
        let col = (cycle % 40) as usize;
        CRT[row][col] = if (state.x - 1..=state.x + 1).contains(&(col as i64)) {
            '#'
        } else {
            '.'
        };

        cycle += 1;

        if cycle == 20 || (cycle - 20) % 40 == 0 {
            signal_strengths.push(state.x * cycle);
        }

        if let Some(line) = lines.next() {
            //println!("{}", line);
            let mut parts = line.split(' ');
            match parts.next().unwrap() {
                "addx" => {
                    state.add_x_buffer.push_back(0);
                    state
                        .add_x_buffer
                        .push_back(parts.next().unwrap().parse::<i64>().unwrap())
                }
                _ => state.add_x_buffer.push_back(0),
            }
        }

        if let Some(add_x) = state.add_x_buffer.pop_front() {
            state.x += add_x
        }

        if state.add_x_buffer.is_empty() && lines.peek().is_none() {
            break;
        }
    }
    println!("{:?}", signal_strengths);
    println!("{:?}", signal_strengths.iter().sum::<i64>());
    CRT.iter().for_each(|row| {
        println!("{}", row.iter().collect::<String>());
    })
}
