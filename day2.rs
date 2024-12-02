use std::io::{Read, Result};

fn get_input() -> Result<Vec<Vec<i32>>> {
    let mut file = std::fs::File::open("input")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    Ok(buf
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|line| line.parse().unwrap_or_default())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>())
}

fn check_constraints(a: i32, b: i32, ord: i32) -> bool {
    if ord == 1 && a > b {
        return false;
    } else if ord == 2 && a < b {
        return false;
    }

    let diff = i32::abs(a - b);

    return 1 <= diff && diff <= 3;
}

fn safe(report: Vec<i32>) -> bool {
    let mut ord = 0;
    for levels in report[..].windows(2) {
        if ord == 0 {
            ord = if levels[0] < levels[1] { 1 } else { 2 };
        }

        if !check_constraints(levels[0], levels[1], ord) {
            return false;
        }
    }
    return true;
}

fn p1() -> Result<()> {
    let input = get_input()?;

    let ans = input
        .into_iter()
        .fold(0, |acc, report| if safe(report) { acc + 1 } else { acc });

    println!("{ans}");
    Ok(())
}

fn p2() -> Result<()> {
    let input = get_input()?;

    let ans = input.into_iter().fold(0, |acc, report| {
        let mut i = 0;
        let n = report.len();
        let mut ord = 0;

        let mut found = true;
        while i < n - 1 {
            if ord == 0 {
                ord = if report[i] < report[i + 1] { 1 } else { 2 };
            }
            if !check_constraints(report[i], report[i + 1], ord) {
                found = false;
                for i in 0..n {
                    let mut clone = report.clone();
                    clone.remove(i);
                    if safe(clone) {
                        found = true;
                        break;
                    }
                }
            }
            i += 1;
        }

        if !found {
            acc
        } else {
            acc + 1
        }
    });

    println!("{ans}");

    Ok(())
}

fn main() -> std::io::Result<()> {
    p1()?;
    p2()
}
