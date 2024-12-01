use std::{collections::HashMap, iter::zip};

fn main() {
    let input = include_str!("./input.txt");

    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let (l, r) = line.split_once(" ").unwrap();
        left.push(l.trim().parse::<i64>().unwrap());
        right.push(r.trim().parse::<i64>().unwrap());
    }

    println!("Part I:  {}", part1(&left, &right));
    println!("Part II: {}", part2(&left, &right));
}

fn part1(left: &Vec<i64>, right: &Vec<i64>) -> i64 {
    let mut l = left.clone();
    l.sort();
    let mut r = right.clone();
    r.sort();

    let mut delta = 0;

    for (l, r) in zip(l, r) {
        delta += (l - r).abs();
    }

    delta
}

fn part2(left: &Vec<i64>, right: &Vec<i64>) -> i64 {
    let mut right_frequencies = HashMap::new();

    for r in right {
        right_frequencies.insert(
            r,
            match right_frequencies.get(r) {
                Some(n) => n + 1,
                None => 1,
            },
        );
    }

    left.iter()
        .map(|l| l * right_frequencies.get(l).or(Some(&0)).unwrap())
        .sum()
}
