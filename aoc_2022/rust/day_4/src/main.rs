use std::fs;

#[derive(Debug, Copy, Clone)]
struct Bounds {
    min: i32,
    max: i32,
}

impl Bounds {
    fn new(left: i32, right: i32) -> Bounds {
        Bounds {
            min: left,
            max: right,
        }
    }

    fn from_input(input: &str) -> Bounds {
        let mut bounds = input.split('-').map(|x| x.parse::<i32>().unwrap());
        Bounds::new(bounds.next().unwrap(), bounds.next().unwrap())
    }

    fn contains(&self, other: Bounds) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn intersects(&self, other: Bounds) -> bool {
        self.max >= other.min && self.min <= other.max
    }
}

fn read_input(input: &str) -> Vec<(Bounds, Bounds)> {
    fs::read_to_string(input)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.split(',').map(|x| Bounds::from_input(x)))
        .map(|mut x| (x.next().unwrap(), x.next().unwrap()))
        .collect::<Vec<(Bounds, Bounds)>>()
}

fn count_contained_bounds(bounds: &Vec<(Bounds, Bounds)>) -> usize {
    bounds
        .iter()
        .map(|x| x.0.contains(x.1) || x.1.contains(x.0))
        .filter(|x| *x)
        .count()
}

fn part_one(input: &Vec<(Bounds, Bounds)>) -> usize {
    count_contained_bounds(&input)
}

fn count_partially_contained_bounds(bounds: &Vec<(Bounds, Bounds)>) -> usize {
    bounds
        .iter()
        .map(|x| x.0.intersects(x.1) || x.1.intersects(x.0))
        .filter(|x| *x)
        .count()
}

fn part_two(input: &Vec<(Bounds, Bounds)>) -> usize {
    count_partially_contained_bounds(&input)
}

fn main() {
    let input = read_input("./src/input.txt");

    println!("Part one: {:?}", part_one(&input));
    println!("Part two: {:?}", part_two(&input));
}
