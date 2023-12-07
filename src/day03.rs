#[derive(Debug)]
struct Number(i32, i32, u32);
#[derive(Debug)]
struct Symbol(i32, i32);

pub fn part1(input: String) -> anyhow::Result<String> {
    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];

    input.lines().enumerate().for_each(|(row, line)| {
        let chars: Vec<char> = line.chars().collect();
        let mut col = 0;
        while col < line.len() {
            match chars[col] {
                x if x.is_digit(10) => {
                    let start = col;
                    let mut val = chars[col].to_digit(10).unwrap();

                    while col + 1 < line.len() && chars[col + 1].is_digit(10) {
                        val = 10 * val + chars[col + 1].to_digit(10).unwrap();
                        col += 1;
                    }

                    numbers.push(Number(row as i32, start as i32, val))
                }
                '.' => {}
                _ => symbols.push(Symbol(row as i32, col as i32)),
            }

            col += 1;
        }
    });

    let part_sum: u32 = numbers
        .iter()
        .filter(|&num| {
            let num_len = num.2.checked_ilog10().unwrap_or(0) + 1;
            let min_row = num.0 - 1;
            let max_row = num.0 + 1;
            let min_col = num.1 - 1;
            let max_col = num.1 + num_len as i32;

            for s in &symbols {
                if (min_row <= s.0) && (s.0 <= max_row) && (min_col <= s.1) && (s.1 <= max_col) {
                    // Part
                    return true;
                }
            }

            // If none match, not a part
            return false;
        })
        .map(|part| part.2)
        .sum();

    Ok(part_sum.to_string())
}

pub fn part2(input: String) -> anyhow::Result<String> {
    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];
    let mut gears: Vec<Symbol> = vec![];

    input.lines().enumerate().for_each(|(row, line)| {
        let chars: Vec<char> = line.chars().collect();
        let mut col = 0;
        while col < line.len() {
            match chars[col] {
                x if x.is_digit(10) => {
                    let start = col;
                    let mut val = chars[col].to_digit(10).unwrap();

                    while col + 1 < line.len() && chars[col + 1].is_digit(10) {
                        val = 10 * val + chars[col + 1].to_digit(10).unwrap();
                        col += 1;
                    }

                    numbers.push(Number(row as i32, start as i32, val))
                }
                '.' => {}
                '*' => {
                    gears.push(Symbol(row as i32, col as i32));
                    symbols.push(Symbol(row as i32, col as i32));
                }
                _ => symbols.push(Symbol(row as i32, col as i32)),
            }

            col += 1;
        }
    });

    let part_numbers: Vec<Number> = numbers
        .into_iter()
        .filter(|num| {
            let num_len = num.2.checked_ilog10().unwrap_or(0) + 1;
            let min_row = num.0 - 1;
            let max_row = num.0 + 1;
            let min_col = num.1 - 1;
            let max_col = num.1 + num_len as i32;

            for s in &symbols {
                if (min_row <= s.0) && (s.0 <= max_row) && (min_col <= s.1) && (s.1 <= max_col) {
                    // Part
                    return true;
                }
            }

            // If none match, not a part
            return false;
        })
        .collect();

    let gear_ration_sum: u32 = gears
        .iter()
        .filter_map(|gear| {
            // Check if this is a valid gear
            let adjacent_parts: Vec<&Number> = part_numbers
                .iter()
                .filter(|num| {
                    let num_len = num.2.checked_ilog10().unwrap_or(0) + 1;
                    let min_row = num.0 - 1;
                    let max_row = num.0 + 1;
                    let min_col = num.1 - 1;
                    let max_col = num.1 + num_len as i32;
                    if (min_row <= gear.0)
                        && (gear.0 <= max_row)
                        && (min_col <= gear.1)
                        && (gear.1 <= max_col)
                    {
                        // Adjacent
                        return true;
                    }

                    // Not adjacent
                    return false;
                })
                .collect();

            if adjacent_parts.len() != 2 {
                return None;
            }

            return Some(adjacent_parts[0].2 * adjacent_parts[1].2);
        })
        .sum();

    return Ok(gear_ration_sum.to_string());
}

#[test]
fn test_part1() {
    let input: String = String::from(
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
    );

    assert_eq!(String::from("4361"), part1(input).unwrap())
}

#[test]
fn test_part2() {
    let input: String = String::from(
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
    );

    assert_eq!(String::from("467835"), part2(input).unwrap())
}
