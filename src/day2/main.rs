fn main() {
    println!("Hello, world!");
    let input = get_input();
    //println!("{:?}", input);
    let lines = parseing(input);
    //println!("{:?}", lines);
    //let p1 = part1(lines);
    //println!("{}", p1);
    let p2 = part2(lines);
    println!("{}", p2);
}

fn parseing(input: &str) -> Vec<(i32, &str)> {
    let mut res = vec![];
    let lines: Vec<&str> = input.lines().collect();
    for l in lines {
        let (a, b) = l.split_once(' ').unwrap();
        let tup = (b.parse().unwrap(), a);
        res.push(tup);
    }
    return res;
}

fn get_input() -> &'static str {
    let input = include_str!("input.txt");
    return input;
}

fn part1(lines: Vec<(i32, &str)>) -> i32 {
    let mut depth = 0;
    let mut long = 0;

    for (num, word) in lines {
        match word {
            "forward" => long += num,
            "down" => depth += num,
            "up" => depth -= num,
            _ => panic!(),
        }
    }
    println!("{},{}", depth, long);
    let res = depth * long;
    return res;
}

fn part2(lines: Vec<(i32, &str)>) -> i32 {
    let mut depth = 0;
    let mut long = 0;
    let mut aim = 0;

    for (num, word) in lines {
        match word {
            "forward" => {
                long += num;
                depth = depth + (num * aim);
            }
            "down" => aim += num,
            "up" => aim -= num,
            _ => panic!(),
        }
    }
    println!("depth: {}, long: {}, aim: {} ", depth, long, aim);
    let res = depth * long;
    return res;
}
