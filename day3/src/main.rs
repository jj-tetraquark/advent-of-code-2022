use std::collections::HashMap;
use std::env;
use std::fs;
use std::iter::zip;
use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Need to provide an input file as a second argument. \
                               Number of arguments is not 2");

    let priority_map: HashMap<char, u32> = zip('a'..='z', 1..=26)
        .chain(zip('A'..='Z', 27..=52))
        .collect();

    let input = fs::read_to_string(&args[1]).unwrap();

    let priority_sum = input
        .lines()
        .map(|line| {
            let (first, last) = line.split_at(line.len() / 2);
            match first.chars().find(|&c| last.contains(c)) {
                Some(found) => *priority_map.get(&found).unwrap(),
                None => panic!("No matching char!"),
            }
        }).sum::<u32>();

    println!("priority sum {:?}", priority_sum);

    let group_priority_sum = {
        let mut sum = 0;
        for chunk in &input.lines().chunks(3) {
            let mut group = chunk.collect::<Vec<&str>>();
            group.sort_by(|bag1, bag2| bag1.len().partial_cmp(&bag2.len()).unwrap());
            match group[0].chars().find(|&c| group[1].contains(c) && group[2].contains(c)) {
                Some(found) => sum += *priority_map.get(&found).unwrap(),
                None => panic!(),
            }
        }
        sum 
    };
    
    println!("group priority sum: {:?}", group_priority_sum);
}
