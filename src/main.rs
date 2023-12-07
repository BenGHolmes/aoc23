use std::fs;

mod day01;
mod day02;
mod day03;

fn main() {
    let jobs: &[(fn(String) -> anyhow::Result<String>, &str)] = &[
        (day01::part1, "inputs/day01.txt"),
        (day01::part2, "inputs/day01.txt"),
        (day02::part1, "inputs/day02.txt"),
        (day02::part2, "inputs/day02.txt"),
        (day03::part1, "inputs/day03.txt"),
        (day03::part2, "inputs/day03.txt"),
    ];

    for j in jobs {
        let input = fs::read_to_string(j.1).unwrap();
        match j.0(input) {
            Ok(result) => println!("OK: {}", result),
            Err(message) => println!("Error: {}", message),
        }
    }
}
