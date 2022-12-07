use std::fs;
use std::env;
use std::ops::{Index, IndexMut};
use std::str;

#[derive(Debug)]
struct Node {
    name: String,
    size: u64,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl Node {
    fn new(name: &str, size: u64) -> Self {
        Self {
            name: String::from(name),
            size,
            parent: None,
            children: vec![]
        }
    }
}


struct Disk {
    disk: Vec<Node>,
}

impl Disk {
    fn new() -> Self {
        Self {
            disk: vec![Node::new("/", 0)]
        }
    }

    fn add(&mut self, parent: usize, name: &str, size: u64) -> usize {
        let idx = self.disk.len(); 
        self.disk.push(Node::new(name, size));
        self.disk[idx].parent = Some(parent);
        self.disk[parent].children.push(idx);
        self.disk[parent].size += size;

        let mut cwd = parent;
        while let Some(parent_) = self.disk[cwd].parent {
            self.disk[parent_].size += size;
            cwd = parent_;
        }

        idx
    }
}

impl Index<usize> for Disk {
    type Output = Node;
    fn index(& self, i: usize) -> &Node {
        &self.disk[i]
    }
}

impl IndexMut<usize> for Disk {
    fn index_mut(&mut self, i: usize) -> &mut Node {
        &mut self.disk[i]
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Need to provide an input file as a second argument. \
                               Number of arguments is not 2");

    let mut cwd = 0;
    let mut disk = Disk::new();
    
    for line in fs::read_to_string(&args[1]).unwrap().lines() {
        let parts = line.split(' ').collect::<Vec<&str>>();
        if parts[0] == "$" { // command
            if parts[1] == "cd" {
                match parts[2] {
                    "/" => cwd = 0,
                    ".." => cwd = disk[cwd].parent.unwrap(),
                    dir => cwd = *disk[cwd].children.iter().find(|&idx| disk[*idx].name == dir).unwrap()
                }
            }
        }
        else { // listing
            match parts[0].parse::<u64>() {
                Ok(size) => { 
                    disk.add(cwd, parts[1], size);
                },
                Err(_) => {
                    disk.add(cwd, parts[1], 0);
                }
            }
        }
    }
    //println!("{:?}", disk.disk);

    let part1 = disk.disk
        .iter()
        .filter(|node| node.size <= 100000 && !node.children.is_empty())
        .fold(0, |acc, node| acc + node.size);

    println!("part1: {}", part1);
    
    let additional_required_free_space = 30000000 - (70000000 - disk[0].size);
    println!("Additional {} required", additional_required_free_space);

    let mut dir_sizes = disk.disk
        .iter()
        .filter(|node| !node.children.is_empty())
        .map(|node| node.size)
        .collect::<Vec<u64>>();
    dir_sizes.sort();

    let part2 = dir_sizes.iter().find(|&size| size >= &additional_required_free_space).unwrap();
    println!("part 2: {}", part2);
}
