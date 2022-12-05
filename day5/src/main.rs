use std::env;
use std::fs;

#[derive(Debug)]
struct Instruction {
    mv: usize,
    from: usize,
    to: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Need to provide an input file as a second argument. \
                               Number of arguments is not 2");

    let input = fs::read_to_string(&args[1]).unwrap();

    let diagram = input
        .lines()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    let mut split = diagram
        .iter()
        .map(|line| {
            line.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|c| c.iter().collect::<String>())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<_>>();

    let mut stacks = vec![
        Vec::<String>::new();
        split
            .pop()
            .unwrap()
            .last()
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap()
    ];

    split.iter().for_each(|row| {
        row.iter().enumerate().for_each(|(i, col)| {
            let crate_ = col
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<String>();
            if !crate_.is_empty() {
                stacks[i].insert(0, crate_);
            }
        })
    });

    let instructions = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            let splits = line.split(' ').collect::<Vec<_>>();
            Instruction {
                mv: splits[1].parse::<usize>().unwrap(),
                from: splits[3].parse::<usize>().unwrap() - 1,
                to: splits[5].parse::<usize>().unwrap() - 1,
            }
        })
        .collect::<Vec<_>>();

    let mut stacks_1 = stacks.clone();
    let mut stacks_2 = stacks;
    instructions.iter().for_each(|cmd| {
        for _ in 0..cmd.mv {
            let to_move = stacks_1[cmd.from].pop().unwrap();
            stacks_1[cmd.to].push(to_move);
        }
        let idx = stacks_2[cmd.from].len() - cmd.mv;
        let mut to_move = stacks_2[cmd.from].split_off(idx);
        stacks_2[cmd.to].append(&mut to_move);
    });

    println!(
        "Top crates: {}",
        stacks_1
            .iter()
            .map(|stack| stack.last().unwrap().clone())
            .collect::<Vec<_>>()
            .join("")
    );

    println!(
        "Top crates 9000 {}",
        stacks_2
            .iter()
            .map(|stack| stack.last().unwrap().clone())
            .collect::<Vec<_>>()
            .join("")
    );
}
