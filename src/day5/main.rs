fn main() {
    println!("Hello, world!");
    let input = get_input();
    let coordinates = parsing(input);
    let p1 = part2(coordinates);
    println!("{}", p1);
}
fn get_input() -> &'static str {
    let input = include_str!("input.txt");
    return input;
}
fn parsing(input: &str) -> Vec<[Coord; 2]> {
    let out = input
        .lines()
        .map(|row| row.split_once(" -> ").unwrap())
        .map(|(start, end)| {
            [start, end]
                .map(|c| c.split_once(',').unwrap())
                .map(|(x, y)| Coord {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                })
        })
        .collect();
    return out;
}
fn part1(coordinates: Vec<[Coord; 2]>) -> i32 {
    let mut lines = vec![];
    for line in coordinates {
        if line[0].x == line[1].x || line[0].y == line[1].y {
            lines.append(&mut hv_line(line));
        }
    }
    count(lines)
}
fn part2(coordinates: Vec<[Coord; 2]>) -> i32 {
    let mut lines = vec![];
    for line in coordinates {
        if line[0].x == line[1].x || line[0].y == line[1].y {
            lines.append(&mut hv_line(line));
        } else {
            lines.append(&mut di_line(line));
        }
    }
    count(lines)
}
fn count(lines: Vec<Coord>) -> i32 {
    let mut mat = [[0u16; 1000]; 1000];
    for line in lines {
        mat[line.y][line.x] += 1;
    }
    println!("{:?}", mat);
    let mut count = 0;
    for y in 0..1000 {
        for x in 0..1000 {
            if mat[y][x] > 1 {
                count += 1;
            }
        }
    }
    count
}
fn hv_line(line: [Coord; 2]) -> Vec<Coord> {
    let mut vec = vec![];
    if line[0].x == line[1].x {
        if line[0].y < line[1].y {
            for i in line[0].y..=line[1].y {
                vec.push(Coord { x: line[0].x, y: i })
            }
        } else {
            for i in line[1].y..=line[0].y {
                vec.push(Coord { x: line[0].x, y: i })
            }
        }
    } else if line[0].y == line[1].y {
        if line[0].x < line[1].x {
            for i in line[0].x..=line[1].x {
                vec.push(Coord { x: i, y: line[0].y })
            }
        } else {
            for i in line[1].x..=line[0].x {
                vec.push(Coord { x: i, y: line[0].y })
            }
        }
    }
    vec
}
fn di_line(line: [Coord; 2]) -> Vec<Coord> {
    let r1: Vec<usize> = if line[0].x > line[1].x {
        (line[1].x..=line[0].x).rev().collect()
    } else {
        (line[0].x..=line[1].x).collect()
    };
    let r2: Vec<usize> = if line[0].y > line[1].y {
        (line[1].y..=line[0].y).rev().collect()
    } else {
        (line[0].y..=line[1].y).collect()
    };
    let range = r1.into_iter().zip(r2.into_iter());
    let mut out = vec![];
    for (x, y) in range {
        out.push(Coord { x, y });
    }
    out
}
struct Coord {
    x: usize,
    y: usize,
}
