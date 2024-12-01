use std::{collections::HashMap, fs, io::Read};

fn get_input() -> std::io::Result<(Vec<i32>, Vec<i32>)> {
    let mut f = fs::File::open("input")?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let mut l1 = Vec::new();
    let mut l2 = Vec::new();

    buf.lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .for_each(|line| {
            l1.push(line[0].parse::<i32>().unwrap_or(0));
            l2.push(line[1].parse::<i32>().unwrap_or(0));
        });
    Ok((l1, l2))
}

fn p1() -> std::io::Result<()> {
    let (mut l1, mut l2) = get_input()?;

    l1.sort_unstable();
    l2.sort_unstable();

    println!(
        "{}",
        l1.into_iter()
            .zip(l2.into_iter())
            .fold(0, |acc, (i, j)| acc + i32::abs(i - j)),
    );

    Ok(())
}

fn p2() -> std::io::Result<()> {
    let (l1, l2) = get_input()?;

    let mut l2_freq = HashMap::new();

    l2.into_iter().for_each(|i| {
        l2_freq.entry(i).and_modify(|f| *f += 1).or_insert(1);
    });

    println!(
        "{}",
        l1.into_iter().fold(0, |acc, curr| {
            acc + (curr * l2_freq.get(&curr).unwrap_or(&0))
        })
    );

    Ok(())
}

fn main() -> std::io::Result<()> {
    p1()?;
    p2()
}
