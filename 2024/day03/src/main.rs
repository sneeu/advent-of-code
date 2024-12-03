use parser::{find_operations, Operation};

mod parser;

fn main() {
    let input = include_str!("input.txt");

    let operations = find_operations(input);

    //
    println!("Part I:  {}", part1(&operations));
    println!("Part II: {}", part2(&operations));
}

fn part1(ops: &[Operation]) -> i64 {
    ops.iter()
        .filter_map(|op| match op {
            Operation::Mul(m) => Some(m.apply()),
            _ => None,
        })
        .sum()
}

fn part2(ops: &[Operation]) -> i64 {
    let ff: Vec<&Operation> = Vec::new();

    ops.iter()
        .fold(ff, |mut acc, op| {
            match (op, acc.last()) {
                (Operation::Mul(_), Some(Operation::Dont)) => {}
                _ => acc.push(op),
            }
            acc
        })
        .into_iter()
        .map(|op| match op {
            Operation::Mul(m) => m.apply(),
            _ => 0,
        })
        .sum()
}
