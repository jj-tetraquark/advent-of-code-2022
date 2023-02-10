use std::fs;
use std::env;
use std::fmt;

#[derive(Clone)]
enum Value {
    Int(u32),
    List(Vec<Value>)
}

type Pair = (Value, Value);

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::List(v) => write!(f, "{:?}", v)
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Need to provide an input file as a second \
                               argument. Number of arguments is not 2");

    let input = fs::read_to_string(&args[1]).unwrap();

    let packet_pairs: Vec<Pair> = input
        .split("\n\n")
        .map(|pair_str| {
            let lines: Vec<&str> = pair_str.lines().collect(); 
            let left = lines[0];
            let right = lines[1];

            (parse_packet(left), parse_packet(right))
        })
        .collect();

    //packet_pairs.iter().for_each(|pair| {
    //    println!("left: {:?}", pair.0);
    //    println!("right: {:?}", pair.1);
    //    println!("");
    //});
    
    let in_order_pairs = packet_pairs.iter().enumerate().filter_map(|(idx, pair)| {                                                                    
        println!("\npair {}:", idx + 1);
        if is_in_order(pair) {
            println!("in order");
            Some(idx + 1)
        }
        else {
            println!("out of order");
            None
        }
    }).collect::<Vec<usize>>();
    
    println!("in order pairs: {:?}", in_order_pairs);
    println!("sum: {}", in_order_pairs.iter().sum::<usize>())

}

fn parse_block(packet_vec: &[String]) -> Value {
    let mut in_sublist: Option<usize> = None;
    let mut open_brace = 0;
    let list = packet_vec.iter().enumerate().filter_map(|(i, el)| {
        match el.as_str() {
            "[" => {
                if !in_sublist.is_some() {
                    in_sublist = Some(i + 1);
                }
                open_brace += 1;
                None
            },
            "]" => {
                if open_brace == 1 {
                    if let Some(start) = in_sublist {
                        in_sublist = None;
                        open_brace -= 1;
                        return Some(parse_block(&packet_vec[start..i]))
                    }
                }
                open_brace -= 1;
                None // end of block
            },
            _ => if in_sublist.is_some() {
                None
            }
            else {
                Some(Value::Int(el.parse::<u32>().unwrap()))
            }
        }

    })
    .collect::<Vec<Value>>();

    Value::List(list)
}

fn parse_packet(packet_str: &str) -> Value {
    let mut num_buf :Vec<char> = Vec::new();
    let packet_vec: Vec<String> = packet_str
        .chars()
        .filter_map(|c| match c {
            i @'0'..='9' => {
                num_buf.push(i);
                None
            },
            b @ '[' => Some(vec![b.to_string()]),
            b @ ']' => Some(vec![num_buf.drain(..).collect(), b.to_string()]),
            ',' => Some(vec![num_buf.drain(..).collect()]),
            _ => panic!()
        })
        .flatten()
        .filter(|s| !s.is_empty())
        .collect();

    println!("{}", packet_str);
    parse_block(&packet_vec[1..])
}

fn is_in_order(pair: &Pair) -> bool {
    println!("Evaluate {:?} vs {:?}", pair.0, pair.1);
    let left = match &pair.0 {
        Value::List(l) => l.clone(),
        Value::Int(i) => vec![Value::Int(*i)]
    };

    let right = match &pair.1 {
        Value::List(l) => l.clone(),
        Value::Int(i) => vec![Value::Int(*i)]
    };

    for i in 0..std::cmp::max(left.len(), right.len()) {
        let left_el = left.get(i);
        let right_el = right.get(i);
        
        match (left_el, right_el) {
            (Some(left_val), Some(right_val)) => {
                match (left_val, right_val) {
                    (Value::Int(left_int), Value::Int(right_int)) => {
                        if left_int > right_int { 
                            println!("left {} > right {}", left_int, right_int);
                            return false
                        }
                        else if left_int < right_int {
                            println!("left {} < right {}", left_int, right_int);
                            return true
                        }
                        else {
                            println!("left {} == right: {}", left_int, right_int);
                        }
                    },
                    (left_sub, right_sub) => if !is_in_order(&(left_sub.clone(), right_sub.clone())) { 
                        println!("left: {:?}, right: {:?} not in order", left_sub, right_sub);
                        return false 
                    }
                }
            },
            (None, Some(_)) => {
                println!("left ran out of values first!");
                return true
            },// left ran out of values first
            (Some(_), None) => {
                println!("right ran out of values first!");
                return false
            }, // right ran out of values first
            (None, None) => return true
        }
    }
    println!("Got to the end, all in order");
    true
}
