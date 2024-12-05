fn main() {
    let input = include_str!("./input.txt");

    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("Part I:  {}", part1(&lines));
    println!("Part II: {}", part2(&lines));
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn mul(self, n: i64) -> Self {
        Self {
            x: self.x * n,
            y: self.y * n,
        }
    }
}

fn directions() -> Vec<Point> {
    vec![
        Point::new(-1, -1),
        Point::new(-1, 0),
        Point::new(-1, 1),
        Point::new(0, -1),
        // Point::new(0, 0),
        Point::new(0, 1),
        Point::new(1, -1),
        Point::new(1, 0),
        Point::new(1, 1),
    ]
}

fn wordsearch_index(wordsearch: &Vec<Vec<char>>, p: Point) -> Option<char> {
    let line = wordsearch.get(p.x as usize)?;
    Some(*(line.get(p.y as usize)?))
}

fn part1(lines: &Vec<Vec<char>>) -> u64 {
    let mut count = 0;

    let directions = directions();

    for (x, line) in lines.iter().enumerate() {
        for (y, ch) in line.iter().enumerate() {
            if let 'X' = ch {
                let xp = Point::new(x as i64, y as i64);

                for d in directions.iter() {
                    if let Some('M') = wordsearch_index(&lines, xp.add(d.mul(1))) {
                        if let Some('A') = wordsearch_index(&lines, xp.add(d.mul(2))) {
                            if let Some('S') = wordsearch_index(&lines, xp.add(d.mul(3))) {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    count
}

fn diagonals(wordsearch: &Vec<Vec<char>>, p: Point) -> String {
    let mut r = vec![];

    for d in [
        Point::new(-1, -1),
        Point::new(-1, 1),
        Point::new(1, 1),
        Point::new(1, -1),
    ] {
        if let Some(ch) = wordsearch_index(wordsearch, p.add(d)) {
            r.push(ch.to_string())
        }
    }

    r.join("")
}

fn part2(lines: &Vec<Vec<char>>) -> u64 {
    let mut count = 0;

    let x_mases = vec!["MMSS", "SMMS", "SSMM", "MSSM"];

    for (x, line) in lines.iter().enumerate() {
        for (y, ch) in line.iter().enumerate() {
            if let 'A' = ch {
                let ap = Point::new(x as i64, y as i64);
                let diags = diagonals(lines, ap);
                if x_mases.contains(&diags.as_str()) {
                    count += 1;
                }
            }
        }
    }

    count
}
