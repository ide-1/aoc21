    let input = get_input();
    let lines = parse(input);
    let p1 = part1(&lines);
    println!("{}", p1);
    let p2 = part2(&lines);
    println!("{}", p2);
}

fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.parse().expect("invalid input"))
        .collect()
}

fn get_input() -> &'static str {
    let input = include_str!("input.txt");
    return input;
}

fn part1(input: &Vec<i32>) -> i32 {
    let mut increased = 0;
    for i in 0..input.len() - 1 {
        if input[i] < input[i + 1] {
            increased += 1;
        }
    }
    return increased;
}

fn part2(input: &Vec<i32>) -> i32 {
    let mut increased = 0;
    let summed_input = sum_parts(input);
    for i in 0..summed_input.len() - 1 {
        if summed_input[i] < summed_input[i + 1] {
            increased += 1;
        }
    }
    return increased;
}

fn sum_parts(input: &Vec<i32>) -> Vec<i32> {
    let mut output = vec![];
    for i in 0..input.len() - input.len() % 3 {
        output.push(input[i] + input[i + 1] + input[i + 2])
    }
    return output;
}
