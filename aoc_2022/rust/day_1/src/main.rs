use std::fs;

fn read_file(path: &str) -> Result<String, String> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(error) => Err(format!("Error while reading file: {}", error)),
    }
}

fn find_most_calories_part_one(input_string: String) -> i64 {
    return split_input(input_string)
        .iter()
        .map(|x| sum_calories(x))
        .max()
        .unwrap();
}

fn find_most_calories_part_two(input_string: String) -> i64 {
    let mut res = split_input(input_string)
        .iter()
        .map(|x| sum_calories(x))
        .collect::<Vec<i64>>();

    res.sort();
    res.reverse();

    return res[0..=2].iter().fold(0, |acc, x| acc + x);
}

fn split_input(input_string: String) -> Vec<Vec<String>> {
    return input_string
        .chars()
        .filter(|x| *x != '\r')
        .collect::<String>()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .iter()
        .map(|x| {
            x.split('\n')
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();
}

fn sum_calories(calorie_list: &Vec<String>) -> i64 {
    return calorie_list
        .into_iter()
        .map(|string| string.parse::<i64>())
        .fold(0, |acc, x| acc + x.unwrap_or(0));
}

fn main() {
    let res = read_file("./src/input.txt")
        .map(find_most_calories_part_one)
        .unwrap();
    println!("{}", res);

    let res2 = read_file("./src/input.txt")
        .map(find_most_calories_part_two)
        .unwrap();

    println!("{}", res2);
}
