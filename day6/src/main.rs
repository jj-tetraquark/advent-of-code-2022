use std::env;
use std::fs;
use std::collections::HashSet;

fn find_start(input: &[char], sequence_length: usize) -> usize {
    let search = input
        .windows(sequence_length)
        .enumerate()
        .find(|&(_i, block)| block.iter().collect::<HashSet<_>>().len() == sequence_length);

    if let Some((idx, _)) = search {
        idx + sequence_length
    }
    else {
        panic!("Could not find marker");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Need to provide an input file as a second argument. \
                               Number of arguments is not 2");

    let input = fs::read_to_string(&args[1]).unwrap().chars().collect::<Vec<char>>();

    let start_of_packet = find_start(&input, 4);
    println!("Start of packet: {}", start_of_packet);
    let start_of_msg = find_start(&input, 14);
    println!("Start of message: {}", start_of_msg);
}
