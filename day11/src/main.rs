use std::env;
use std::fs;
use std::collections::VecDeque;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;
use rug::Integer;
use rug::Complete;
use rand::prelude::*;

struct Monke {
    items: VecDeque<Integer>,
    operation: Box<dyn Fn(&Integer) -> Integer>,
    throws_to: Box<dyn Fn(&Integer) -> usize>,
    items_handled_count: u64
}

impl Monke {
    fn new(info: &str, worry_div: u64) -> Self {
        lazy_static! {
            static ref START_RE: Regex = Regex::new(
                r"Starting items: ([0-9]*(,\s[0-9]*)*)\n")
                .unwrap();
            static ref OP_RE: Regex = Regex::new(
                r"Operation: new = old ([\*\+]) ([a-z0-9]*)\n")
                .unwrap();
            static ref TEST_RE: Regex = Regex::new(
                r"Test: divisible by ([0-9]*)\n\s*[a-zA-Z\s:]*([0-9]*)\n\s*[a-zA-Z\s:]*([0-9]*)")
                .unwrap();
        }
        let items = START_RE.captures(info)
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str()
                        .split(", ")
                        .map(|num| num.parse::<Integer>().unwrap())
                        .collect();

        let operation: Box<dyn Fn(&Integer) -> Integer> = {
            let cap = OP_RE.captures(info).unwrap();
            let op = cap.get(1).unwrap().as_str();
            let rhs_str = cap.get(2).unwrap().as_str();
            
            match op {
                "*" => {
                    if let Ok(rhs) = rhs_str.parse::<u64>() {
                        Box::new(move |item| (item * rhs).complete()/worry_div)
                    }
                    else {
                        Box::new(move |item| (item * item).complete()/worry_div)
                    }
                },
                "+" => {
                    if let Ok(rhs) = rhs_str.parse::<u64>() {
                        Box::new(move |item| (item + rhs).complete()/worry_div)
                    }
                    else {
                        Box::new(move |item| (item + item).complete()/worry_div)
                    }
                },
                _ => panic!("unrecognised op '{}'", op)
            }
        };
        
        let throws_to: Box<dyn Fn(&Integer) -> usize> = {
            let cap = TEST_RE.captures(info).unwrap();
            let division = cap.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let true_monke = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let false_monke = cap.get(3).unwrap().as_str().parse::<usize>().unwrap();
    
            //println!("if div by {} throw to {} else {}", division, true_monke, false_monke);
            Box::new(move |item| if (item % division).complete() == 0 { true_monke } else { false_monke })
        };

        Self {
            items,
            operation,
            throws_to,
            items_handled_count: 0
        }

    }
}

impl fmt::Debug for Monke {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Monke {{ items: {:?}, items_handled_count: {}}}", self.items, self.items_handled_count)
    }
}

fn factorise_to_primes(n: &Integer) -> Vec<Integer> {
    println!("factorising: {}", n);
    let mut primes = Vec::new();
    let mut cur_prime = n.clone();
    loop {
        let new_prime = pollard_rho(&cur_prime);
        cur_prime /= &new_prime;
        primes.push(new_prime);
        if cur_prime == 1 {
            break;
        }
    }
    primes
}

fn pollard_rho(n: &Integer) -> Integer {
    if *n == 1 {
        return Integer::from(1);
    }
    if n.mod_u(2) == Integer::ZERO {
        return Integer::from(2);
    }
    if n.is_probably_prime(30) == rug::integer::IsPrime::Yes {
        return n.clone()
    }
    
    let mut rng = rand::thread_rng();
    
    let mut x: Integer = Integer::from(rng.gen::<u64>() + 2);
    let mut y = x.clone();

    let c: Integer = Integer::from(rng.gen::<u64>() + 1);
    let mut d = Integer::from(1);

    let two = Integer::from(2);

    while d == 1 {
        x = (x.pow_mod(&two, &n).unwrap() + &c + n) % n;
        y = (y.pow_mod(&two, &n).unwrap() + &c + n) % n;
        y = (y.pow_mod(&two, &n).unwrap() + &c + n) % n;

        d = (&x-&y).complete().abs().gcd(&n);

        if &d == n {
            //println!("{}", n);
            return pollard_rho(n)
        }
    }
    d
}

// Items stored as factorised primes
type Item = Vec<Integer>;

struct SmartMonke {
    items: VecDeque<Item>,
    operation: Box<dyn Fn(&mut Item)>,
    throws_to: Box<dyn Fn(&Item) -> usize>,
    items_handled_count: u64
}

impl SmartMonke {
    fn new(info: &str) -> Self {
        lazy_static! {
            static ref START_RE: Regex = Regex::new(
                r"Starting items: ([0-9]*(,\s[0-9]*)*)\n")
                .unwrap();
            static ref OP_RE: Regex = Regex::new(
                r"Operation: new = old ([\*\+]) ([a-z0-9]*)\n")
                .unwrap();
            static ref TEST_RE: Regex = Regex::new(
                r"Test: divisible by ([0-9]*)\n\s*[a-zA-Z\s:]*([0-9]*)\n\s*[a-zA-Z\s:]*([0-9]*)")
                .unwrap();
        }
        let items = START_RE.captures(info)
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str()
                        .split(", ")
                        .map(|num| factorise_to_primes(&num.parse::<Integer>().unwrap()))
                        .collect();

        let operation: Box<dyn Fn(&mut Item)> = {
            let cap = OP_RE.captures(info).unwrap();
            let op = cap.get(1).unwrap().as_str();
            let rhs_str = cap.get(2).unwrap().as_str();
            
            match op {
                "*" => {
                    if let Ok(rhs) = rhs_str.parse::<u64>() {
                        Box::new(move |item| item.push(Integer::from(rhs)))
                    }
                    else {
                        Box::new(move |item| item.extend(item.clone().into_iter()))
                    }
                },
                "+" => {
                    if let Ok(rhs) = rhs_str.parse::<u64>() {
                        Box::new(move |item| {
                            let num = Integer::product(item.iter()).complete();
                            println!("{}", num);
                            *item = factorise_to_primes(
                                &Integer::from(num + rhs)
                                )
                        })
                    }
                    else {
                        Box::new(move |item| item.push(Integer::from(2)))
                    }
                },
                _ => panic!("unrecognised op '{}'", op)
            }
        };
        
        let throws_to: Box<dyn Fn(&Item) -> usize> = {
            let cap = TEST_RE.captures(info).unwrap();
            let division = cap.get(1).unwrap().as_str().parse::<Integer>().unwrap();
            let true_monke = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let false_monke = cap.get(3).unwrap().as_str().parse::<usize>().unwrap();
    
            Box::new(move |item| if item.contains(&division) { true_monke } else { false_monke })
        };

        Self {
            items,
            operation,
            throws_to,
            items_handled_count: 0
        }

    }
}

impl fmt::Debug for SmartMonke {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Monke {{ items: {:?}, items_handled_count: {}}}", self.items, self.items_handled_count)
    }
}

struct ModMonke {
    items: VecDeque<u64>,
    operation: Box<dyn Fn(&u64) -> u64>,
    throws_to: Box<dyn Fn(&u64) -> usize>,
    items_handled_count: u64
}

impl ModMonke {
    fn new(info: &str, mod_prod: u64) -> Self {
        lazy_static! {
            static ref START_RE: Regex = Regex::new(
                r"Starting items: ([0-9]*(,\s[0-9]*)*)\n")
                .unwrap();
            static ref OP_RE: Regex = Regex::new(
                r"Operation: new = old ([\*\+]) ([a-z0-9]*)\n")
                .unwrap();
            static ref TEST_RE: Regex = Regex::new(
                r"Test: divisible by ([0-9]*)\n\s*[a-zA-Z\s:]*([0-9]*)\n\s*[a-zA-Z\s:]*([0-9]*)")
                .unwrap();
        }
        let items = START_RE.captures(info)
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str()
                        .split(", ")
                        .map(|num| num.parse::<u64>().unwrap())
                        .collect();

        let operation: Box<dyn Fn(&u64) -> u64> = {
            let cap = OP_RE.captures(info).unwrap();
            let op = cap.get(1).unwrap().as_str();
            let rhs_str = cap.get(2).unwrap().as_str();
            
            match op {
                "*" => {
                    if let Ok(rhs) = rhs_str.parse::<u64>() {
                        Box::new(move |item| (item * rhs) % mod_prod)
                    }
                    else {
                        Box::new(move |item| (item * item) % mod_prod)
                    }
                },
                "+" => {
                    if let Ok(rhs) = rhs_str.parse::<u64>() {
                        Box::new(move |item| (item + rhs) % mod_prod)
                    }
                    else {
                        Box::new(move |item| (item + item) % mod_prod)
                    }
                },
                _ => panic!("unrecognised op '{}'", op)
            }
        };
        
        let throws_to: Box<dyn Fn(&u64) -> usize> = {
            let cap = TEST_RE.captures(info).unwrap();
            let division = cap.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let true_monke = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let false_monke = cap.get(3).unwrap().as_str().parse::<usize>().unwrap();
    
            //println!("if div by {} throw to {} else {}", division, true_monke, false_monke);
            Box::new(move |item| if (item % division) == 0 { true_monke } else { false_monke })
        };

        Self {
            items,
            operation,
            throws_to,
            items_handled_count: 0
        }

    }
}
fn run_simulation(monke_raw: &Vec<&str>, iterations: u64, worry_div: u64) -> Integer {
    let mut all_monke: Vec<ModMonke> = monke_raw
        .iter()
        .map(|info| ModMonke::new(info, worry_div))
        .collect();

    for j in 0..iterations {
        println!("{}", j);
        for i in 0..all_monke.len() {
            while let Some(item) = all_monke[i].items.pop_front() {
                let new_item_worry = (all_monke[i].operation)(&item);
                let throws_to = (all_monke[i].throws_to)(&new_item_worry);
                all_monke[throws_to].items.push_back(new_item_worry);
                all_monke[i].items_handled_count += 1;
            }
        }
    }

    let mut activity: Vec<u64> = all_monke.iter().map(|monke| monke.items_handled_count).collect();
    activity.sort();
    activity.iter().rev().take(2).product::<Integer>()
}

fn run_simulation2(monke_raw: &Vec<&str>, iterations: u64) -> Integer {
    let mut all_monke: Vec<SmartMonke> = monke_raw
        .iter()
        .map(|info| SmartMonke::new(info))
        .collect();

    for j in 0..iterations {
        println!("{}", j);
        for i in 0..all_monke.len() {
            while let Some(mut item) = all_monke[i].items.pop_front() {
                (all_monke[i].operation)(&mut item);
                let throws_to = (all_monke[i].throws_to)(&item);
                all_monke[throws_to].items.push_back(item);
                all_monke[i].items_handled_count += 1;
            }
        }
    }

    println!("{:?}", all_monke);
    let mut activity: Vec<u64> = all_monke.iter().map(|monke| monke.items_handled_count).collect();
    activity.sort();
    activity.iter().rev().take(2).product::<Integer>()
}


fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Need to provide an input file as a second \
                               argument. Number of arguments is not 2");

    let input = fs::read_to_string(&args[1]).unwrap();
    let monke_raw = input.split("\n\n").collect::<Vec<&str>>();

    // println!("Monkey business: {}", run_simulation(&monke_raw, 20, 3));
    let mod_prod = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;
    println!("Monkey business 2: {}", run_simulation(&monke_raw, 10000, mod_prod));
}
