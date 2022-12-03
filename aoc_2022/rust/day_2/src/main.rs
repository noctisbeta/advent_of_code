use core::panic;
use std::fs;

#[derive(Copy, Clone, Debug)]
enum RPSResult {
    Win,
    Lose,
    Draw,
}

// derive copy
#[derive(Copy, Clone, Debug)]
struct RPSTurn {
    computer: char,
    player: char,
}

impl RPSTurn {
    fn new(c: char, p: char) -> RPSTurn {
        RPSTurn {
            computer: c,
            player: p,
        }
    }

    fn from_input(input: &str) -> RPSTurn {
        RPSTurn::new(input.chars().nth(0).unwrap(), input.chars().nth(2).unwrap())
    }
}

fn check_win(turn: RPSTurn) -> RPSResult {
    match turn.computer {
        'A' => {
            if turn.player == 'Y' {
                return RPSResult::Win;
            } else if turn.player == 'Z' {
                return RPSResult::Lose;
            } else {
                return RPSResult::Draw;
            }
        }
        'B' => {
            if turn.player == 'Z' {
                return RPSResult::Win;
            } else if turn.player == 'X' {
                return RPSResult::Lose;
            } else {
                return RPSResult::Draw;
            }
        }
        'C' => {
            if turn.player == 'X' {
                return RPSResult::Win;
            } else if turn.player == 'Y' {
                return RPSResult::Lose;
            } else {
                return RPSResult::Draw;
            }
        }
        _ => panic!("Invalid computer choice"),
    };
}

fn get_choice_score(choice: char) -> i64 {
    match choice {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("Invalid choice"),
    }
}

fn get_win_score(result: RPSResult) -> i64 {
    match result {
        RPSResult::Win => 6,
        RPSResult::Draw => 3,
        RPSResult::Lose => 0,
    }
}

fn count_score(input: String) -> i64 {
    input
        .split('\n')
        .map(RPSTurn::from_input)
        .map(|turn| (check_win(turn), get_choice_score(turn.player)))
        .map(|(result, choice_score)| (get_win_score(result), choice_score))
        .fold(0, |acc, x| acc + x.0 + x.1)
}

fn get_outcome(code: char) -> RPSResult {
    match code {
        'Z' => RPSResult::Win,
        'X' => RPSResult::Lose,
        'Y' => RPSResult::Draw,
        _ => panic!("Invalid input"),
    }
}

fn get_right_move(computer: char, outcome: RPSResult) -> char {
    match computer {
        'A' => match outcome {
            RPSResult::Win => 'Y',
            RPSResult::Lose => 'Z',
            RPSResult::Draw => 'X',
        },
        'B' => match outcome {
            RPSResult::Win => 'Z',
            RPSResult::Lose => 'X',
            RPSResult::Draw => 'Y',
        },
        'C' => match outcome {
            RPSResult::Win => 'X',
            RPSResult::Lose => 'Y',
            RPSResult::Draw => 'Z',
        },
        _ => panic!("Invalid input"),
    }
}

fn count_part_two_score(input: String) -> i64 {
    input
        .split('\n')
        .map(RPSTurn::from_input)
        .map(|turn| (turn.computer, get_outcome(turn.player)))
        .map(|(comp, out)| (get_right_move(comp, out), out))
        .map(|(right_move, out)| (get_choice_score(right_move), get_win_score(out)))
        .fold(0, |acc, x| acc + x.0 + x.1)
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();

    // println!("Part 1: {}", count_score(input));
    println!("Part 2: {}", count_part_two_score(input));
}
