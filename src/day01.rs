pub fn part1(input: String) -> anyhow::Result<String> {
    let res: u32 = input
        .lines()
        .map(|line| {
            let nums: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            nums[0] * 10 + nums[nums.len() - 1]
        })
        .sum();

    Ok(res.to_string())
}

pub fn part2(input: String) -> anyhow::Result<String> {
    let numbers: Vec<&str> = vec![
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine",
    ];

    let res: u32 = input
        .lines()
        .map(|line| {
            let left_idx = numbers
                .iter()
                .filter_map(|num| line.find(num))
                .min()
                .unwrap();

            let line_rev: String = line.chars().rev().collect();
            let n = line.len();
            let right_idx = numbers
                .iter()
                .map(|num| num.chars().rev().collect::<String>())
                .filter_map(|num| line_rev.find(&num).map(|idx| n - idx - num.len()))
                .max()
                .unwrap();

            let left = parse(line, left_idx);
            let right = parse(line, right_idx);

            left * 10 + right
        })
        .sum();

    Ok(res.to_string())
}

fn parse(text: &str, idx: usize) -> u32 {
    let chars: Vec<char> = text.chars().collect();
    if chars[idx].is_digit(10) {
        return chars[idx].to_digit(10).unwrap();
    }

    return match chars[idx..idx + 2] {
        ['z', 'e'] => 0,
        ['o', 'n'] => 1,
        ['t', 'w'] => 2,
        ['t', 'h'] => 3,
        ['f', 'o'] => 4,
        ['f', 'i'] => 5,
        ['s', 'i'] => 6,
        ['s', 'e'] => 7,
        ['e', 'i'] => 8,
        ['n', 'i'] => 9,
        _ => panic!("not a digit"),
    };
}

#[test]
fn test_part2() {
    let input: String = String::from("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen");

    assert_eq!(String::from("281"), part2(input).unwrap())
}
