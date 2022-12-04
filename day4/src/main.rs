use std::env;
use std::fs;

type ElfPair = Vec<Vec<u32>>;

fn parse_line(line: &str) -> ElfPair {
    line.split(',')
        .map(|elf| elf.split('-').map(|x| x.parse::<u32>().unwrap()).collect())
        .collect()
}

fn complete_overlap(elf_pair: ElfPair) -> bool {
    (elf_pair[0][0] <= elf_pair[1][0] && elf_pair[0][1] >= elf_pair[1][1])
        || (elf_pair[1][0] <= elf_pair[0][0] && elf_pair[1][1] >= elf_pair[0][1])
}

fn partial_overlap(elf_pair: ElfPair) -> bool {
    (elf_pair[1][0]..=elf_pair[1][1]).contains(&elf_pair[0][0])
        || (elf_pair[1][0]..=elf_pair[1][1]).contains(&elf_pair[0][1])
        || (elf_pair[0][0]..=elf_pair[0][1]).contains(&elf_pair[1][0])
        || (elf_pair[0][0]..=elf_pair[0][1]).contains(&elf_pair[1][1])
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Need to provide an input file as a second argument. \
                               Number of arguments is not 2");

    let input = fs::read_to_string(&args[1]).unwrap();

    let overlaps = input.lines().fold((0, 0), |acc, line| {
        let section_ranges = parse_line(line);
        (
            acc.0 + u32::from(complete_overlap(section_ranges.clone())),
            acc.1 + u32::from(partial_overlap(section_ranges)),
        )
    });

    println!("Overlapping assignments: {}", overlaps.0);
    println!("Partial overlapping assignments: {}", overlaps.1);
}
