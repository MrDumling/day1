use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn get_input() -> Lines<BufReader<File>> {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines()
}

fn section_sums(input: Lines<BufReader<File>>) -> Vec<u64> {
    let mut output = Vec::new();
    let mut current_sum: Option<u64> = None;

    for line in input {
        let line = line.unwrap();
        if line.is_empty() {
            output.push(current_sum.unwrap());
            current_sum = None;
            continue;
        }

        let parsed_value = line.parse::<u64>().unwrap();
        current_sum = Some(if let Some(x) = current_sum {
            parsed_value + x
        } else {
            parsed_value
        });
    }

    if let Some(x) = current_sum {
        output.push(x);
    }

    output
}

fn puzzle_1() {
    let max = section_sums(get_input())
        .into_iter()
        .reduce(u64::max)
        .unwrap();

    println!("{max}");
}

fn puzzle_2() {
    let mut x = section_sums(get_input())
        .into_iter()
        .collect::<Vec<u64>>();

    x.sort_by(|a, b| b.cmp(a));

    println!("{}", x[0] + x[1] + x[2]);
}

fn main() {
    puzzle_1();
    puzzle_2();
}
