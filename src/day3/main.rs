fn main() {
    println!("Hello, world!");
    let input = get_input();
    let lines = parse(input);
    let p1 = part2(lines);
    println!("{}", p1)
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_input() -> &'static str {
    let input = include_str!("input.txt");
    return input;
}

fn part1(input: Vec<Vec<char>>) -> i32 {
    let mut ones = 0;
    let mut zeroes = 0;
    let mut gamma = String::new();
    let mut eps = String::new();
    for i in 0..input[0].len() {
        for j in 0..input.len() {
            if input[j][i] == '1' {
                ones += 1;
            } else {
                zeroes += 1;
            }
        }
        if ones > zeroes {
            gamma.push('1');
            eps.push('0');
        } else {
            gamma.push('0');
            eps.push('1');
        }
        ones = 0;
        zeroes = 0;
    }
    let res = i32::from_str_radix(&gamma, 2).unwrap() * i32::from_str_radix(&eps, 2).unwrap();
    return res;
}

fn part2(input: Vec<Vec<char>>) -> i32 {
    let mut ones = 0;
    let mut zeroes = 0;
    let mut oxygen = String::new();
    let mut co2 = String::new();
    let mut retox = input.clone();
    let mut retco = input.clone();
    for i in 0..retox[0].len() {
        for j in 0..retox.len() {
            if retox[j][i] == '1' {
                ones += 1;
            } else {
                zeroes += 1;
            }
        }
        if ones >= zeroes {
            retox.retain(|x| x[i] == '1');
        } else {
            retox.retain(|x| x[i] == '0');
        }
        if retox.len() == 1 {
            break;
        }
        ones = 0;
        zeroes = 0;
    }
    for i in 0..retco[0].len() {
        for j in 0..retco.len() {
            if retco.len() == 1 {
                break;
            }
            if retco[j][i] == '1' {
                ones += 1;
            } else {
                zeroes += 1;
            }
        }
        if zeroes <= ones {
            retco.retain(|x| x[i] == '0');
        } else {
            retco.retain(|x| x[i] == '1');
        }
        if retco.len() == 1 {
            break;
        }
        ones = 0;
        zeroes = 0;
    }
    for &i in &retox[0] {
        oxygen.push(i);
    }

    for &j in &retco[0] {
        co2.push(j);
    }
    println!("{}, {}", oxygen, co2);
    let res = i32::from_str_radix(&oxygen, 2).unwrap() * i32::from_str_radix(&co2, 2).unwrap();
    return res;
}
