use std::{
    fs,
    io::{Read, Result},
};

fn p1(input: &Vec<Vec<char>>) {
    let mut ans = 0;

    let dirs = [
        [0, 1],
        [1, 1],
        [1, 0],
        [1, -1],
        [0, -1],
        [-1, -1],
        [-1, 0],
        [-1, 1],
    ];

    let target = ['X', 'M', 'A', 'S'];

    for i in input.iter() {
        println!("{i:?}");
    }

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            for [x, y] in dirs.iter() {
                let mut m = i as i32;
                let mut n = j as i32;
                let mut ok = true;

                for i in 0..4 {
                    if 0 <= m
                        && m < input.len() as i32
                        && 0 <= n
                        && n < input.len() as i32
                        && input[m as usize][n as usize] == target[i]
                    {
                        m += x;
                        n += y;
                    } else {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    ans += 1;
                }
            }
        }
    }

    println!("{ans}");
}

fn p2(input: &Vec<Vec<char>>) {
    let mut ans = 0;
    for i in 1..input.len() - 1 {
        for j in 1..input[0].len() - 1 {
            if input[i][j] == 'A' {
                let diag_a = (input[i - 1][j - 1] == 'M' && input[i + 1][j + 1] == 'S')
                    || (input[i - 1][j - 1] == 'S' && input[i + 1][j + 1] == 'M');
                let diag_b = (input[i - 1][j + 1] == 'M' && input[i + 1][j - 1] == 'S')
                    || (input[i - 1][j + 1] == 'S' && input[i + 1][j - 1] == 'M');

                if diag_a && diag_b {
                    ans += 1;
                }
            }
        }
    }

    println!("{ans}");
}
fn main() -> Result<()> {
    let mut file = fs::File::open("input")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let input = buf
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    p1(&input);
    p2(&input);
    Ok(())
}
