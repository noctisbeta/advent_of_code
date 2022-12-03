use std::fs;

fn read_input(input: &str) -> Vec<String> {
    fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
}

fn split_backpack(input: String) -> (String, String) {
    (
        input
            .chars()
            .take(input.chars().count() / 2)
            .collect::<String>(),
        input
            .chars()
            .skip(input.chars().count() / 2)
            .collect::<String>(),
    )
}

fn find_common_letter(backpack1: String, backpack2: String) -> char {
    backpack1
        .chars()
        .map(|letter| (letter, backpack2.contains(letter)))
        .filter(|(_, found)| *found)
        .last()
        .expect("There should be a common leter")
        .0
}

fn encode_letter((letter, is_upper): (char, bool)) -> u8 {
    match is_upper {
        true => letter as u8 - 38,
        false => letter as u8 - 96,
    }
}

fn solve_part_one(backpacks: Vec<String>) -> i64 {
    backpacks
        .iter()
        .map(|line| split_backpack(line.to_string()))
        .map(|(backpack1, backpack2)| find_common_letter(backpack1, backpack2))
        .map(|letter| (letter, letter.is_uppercase()))
        .map(encode_letter)
        .fold(0, |acc, letter| acc + (letter as i64))
}

fn make_groups_of_three(input: Vec<String>) -> Vec<Vec<String>> {
    input
        .chunks(3)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<Vec<String>>>()
}

fn find_common_letter_two(group: Vec<String>) -> char {
    group[0]
        .chars()
        .map(|letter| (letter, group[1].contains(letter), group[2].contains(letter)))
        .filter(|(_, found, found2)| *found && *found2)
        .last()
        .expect("There should be a common leter")
        .0
}

fn solve_part_two(backpacks: Vec<String>) -> i64 {
    make_groups_of_three(backpacks)
        .iter()
        .map(|group| find_common_letter_two(group.to_vec()))
        .map(|letter| (letter, letter.is_uppercase()))
        .map(encode_letter)
        .fold(0, |acc, letter| acc + (letter as i64))
}

fn main() {
    let input = read_input("./src/input.txt");
    // println!("Part one: {:?}", solve_part_one(input));
    println!("Part two: {:?}", solve_part_two(input));
}
