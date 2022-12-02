use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(PartialEq, Clone)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

type Lookup<'a> = HashMap<&'a str, Play>;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Need to provide an input file as a second argument. \
                               Number of arguments is not 2");

    let strat_guide = fs::read_to_string(&args[1]).unwrap();

    let opponent_lookup =
        Lookup::from([("A", Play::Rock), ("B", Play::Paper), ("C", Play::Scissors)]);

    let player_lookup =
        Lookup::from([("X", Play::Rock), ("Y", Play::Paper), ("Z", Play::Scissors)]);

    let total_score = strat_guide.lines().fold(0, |acc, line| {
        let mut parts = line.split(' ');
        let opponent = opponent_lookup.get(parts.next().unwrap()).unwrap();
        let player = player_lookup.get(parts.next().unwrap()).unwrap();
        acc + encounter_score(opponent, player)
    });

    println!("part 1 score: {}", total_score);

    let total_score2 = strat_guide.lines().fold(0, |acc, line| {
        let mut parts = line.split(' ');
        let opponent = opponent_lookup.get(parts.next().unwrap()).unwrap();
        let player = &get_play(opponent, parts.next().unwrap());
        acc + encounter_score(opponent, player)
    });

    println!("part 2 score: {}", total_score2)
}

fn get_win(vs: &Play) -> Play {
    match vs {
        Play::Rock => Play::Paper,
        Play::Paper => Play::Scissors,
        Play::Scissors => Play::Rock,
    }
}

fn get_loss(vs: &Play) -> Play {
    match vs {
        Play::Rock => Play::Scissors,
        Play::Paper => Play::Rock,
        Play::Scissors => Play::Paper,
    }
}

fn get_play(opponent: &Play, code: &str) -> Play {
    match code {
        "X" => get_loss(opponent),
        "Y" => opponent.clone(),
        "Z" => get_win(opponent),
        _ => panic!(),
    }
}

fn encounter_score(opponent: &Play, player: &Play) -> u32 {
    let is_victory = player == &get_win(opponent);
    let is_draw = opponent == player;
    let outcome_score = if is_victory { 6 } else if is_draw { 3 } else { 0 };

    let shape_score = match player {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissors => 3,
    };
    outcome_score + shape_score
}
