fn main() {
    println!("Hello, world!");
    let input = get_input();
    let (bingo_in, boards) = parsing(input);
    let p1 = part2(bingo_in, boards);
    println!("{}", p1);
}

fn get_input() -> &'static str {
    let input = include_str!("input.txt");
    return input;
}

fn parsing(input: &str) -> (Vec<i32>, Vec<Board>) {
    let bingo_in: Vec<i32> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let boards: Vec<Board> = input
        .split("\n\n")
        .skip(1)
        .map(|board| {
            board
                .lines()
                .map(|row| {
                    row.split_whitespace()
                        .map(|element| element.parse().unwrap())
                        .collect()
                })
                .collect()
        })
        .map(|board| Board {
            marked: Default::default(),
            numbers: board,
        })
        .collect();
    (bingo_in, boards)
}

fn part1(bingo_in: Vec<i32>, mut boards: Vec<Board>) -> i32 {
    for n in bingo_in {
        for b in &mut boards {
            b.check(n);
            if b.has_won() {
                return b.count_score(n);
            }
        }
    }
    return 0;
}

fn part2(bingo_in: Vec<i32>, mut boards: Vec<Board>) -> i32 {
    for n in bingo_in {
        for b in &mut boards {
            b.check(n);
        }

        if boards.len() != 1 {
            boards.retain(|b| !b.has_won());
        }
        if boards.len() == 1 && boards[0].has_won() {
            return boards[0].count_score(n);
        }
    }
    return 0;
}

struct Board {
    marked: [[bool; 5]; 5],
    numbers: Vec<Vec<i32>>,
}

impl Board {
    fn check(&mut self, num: i32) {
        for x in 0..5 {
            for y in 0..5 {
                if num == self.numbers[y][x] {
                    self.marked[y][x] = true;
                    return;
                }
            }
        }
    }

    fn has_won(&self) -> bool {
        for y in 0..5 {
            if self.marked[y].iter().all(|&n| n) {
                return true;
            }
        }
        for x in 0..5 {
            let mut col = [false; 5];
            for y in 0..5 {
                col[y] = self.marked[y][x];
            }
            if col.iter().all(|&n| n) {
                return true;
            }
        }
        return false;
    }

    fn count_score(&self, num: i32) -> i32 {
        let mut counter = 0;
        for y in 0..5 {
            for x in 0..5 {
                if !self.marked[y][x] {
                    counter = counter + self.numbers[y][x];
                }
            }
        }
        println!("{},{}", num, counter);
        num * counter
    }
}
