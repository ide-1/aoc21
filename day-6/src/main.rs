use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let input = get_input();
    let fishes = parsing(input);
    let p1 = part1(fishes);
    println!("{}", p1);
}

fn get_input() -> &'static str {
    let input = include_str!("input.txt");
    return input;
}

fn parsing(input: &str) -> HashMap<i32, u64> {
    let vec: Vec<i32> = input.split(',').map(|x| x.parse().unwrap()).collect();
    let mut fishes = HashMap::from([
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (8, 0),
    ]);
    for i in vec {
        fishes.insert(i, fishes[&i] + 1);
    }

    fishes
}

fn day(fishes: &mut HashMap<i32, u64>) {
    let tmp = fishes[&0];
    for i in 0..8 {
        fishes.insert(i, fishes[&(i + 1)]);
    }
    *fishes.get_mut(&6).unwrap() += tmp;
    *fishes.get_mut(&8).unwrap() = tmp;
}

fn part1(mut fishes: HashMap<i32, u64>) -> u64 {
    for i in 0..256 {
        day(&mut fishes);
    }
    fishes.values().copied().sum()
}
