fn main() {
    let input = include_str!("input.txt");

    let reports = input
        .lines()
        .map(|l| {
            l.split(" ")
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<_>>();

    println!("Part I:  {}", part1(&reports));
    println!("Part II: {}", part2(&reports));
}

fn gaps(ns: &[i64]) -> Vec<i64> {
    ns.windows(2).map(|w| w[0] - w[1]).collect()
}

fn ascending(ns: &[i64]) -> bool {
    gaps(ns).iter().all(|g| *g > 0)
}

fn descending(ns: &[i64]) -> bool {
    gaps(ns).iter().all(|g| *g < 0)
}

fn abs_gaps(ns: &[i64]) -> Vec<i64> {
    gaps(ns).iter().map(|n| n.abs()).collect()
}

fn largest_gap(ns: &[i64]) -> i64 {
    *abs_gaps(ns).iter().max().unwrap()
}

fn safe(report: &[i64]) -> bool {
    (ascending(report) || descending(report)) && largest_gap(report) <= 3
}

fn safe2(report: &[i64]) -> bool {
    for index in 0..report.len() {
        // Build a new "reports" with each of the "levels" removed once
        let mut left: Vec<_> = report[..index].into();
        let mut right: Vec<_> = report[index + 1..].into();
        left.append(&mut right);

        if safe(&left) {
            return true;
        }
    }

    false
}

fn part1(reports: &[Vec<i64>]) -> i64 {
    reports.iter().filter(|report| safe(report)).count() as i64
}

fn part2(reports: &[Vec<i64>]) -> i64 {
    reports.iter().filter(|report| safe2(report)).count() as i64
}
