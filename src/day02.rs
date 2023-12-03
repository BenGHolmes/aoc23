use std::collections::HashMap;

pub fn part1(input: String) -> anyhow::Result<String> {
    let color_counts: HashMap<String, i32> = HashMap::from([
        ("red".to_owned(), 12),
        ("green".to_owned(), 13),
        ("blue".to_owned(), 14),
    ]);

    let res: usize = input
        .lines()
        .enumerate()
        .filter_map(|(idx, game)| {
            let sets = game.split(": ").skip(1).next().unwrap();

            let ok = sets.split("; ").all(|set| {
                set.split(", ").all(|count| {
                    let mut it = count.split(' ');
                    let num: i32 = it.next().unwrap().parse().unwrap();
                    let color = it.next().unwrap();

                    num <= color_counts[color]
                })
            });

            if ok {
                Some(idx + 1)
            } else {
                None
            }
        })
        .sum();

    return Ok(res.to_string());
}

pub fn part2(input: String) -> anyhow::Result<String> {
    let res: i32 = input
        .lines()
        .map(|game| {
            let sets = game.split(": ").skip(1).next().unwrap();

            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            sets.split("; ").for_each(|set| {
                set.split(", ").for_each(|count| {
                    let mut it = count.split(' ');
                    let num: i32 = it.next().unwrap().parse().unwrap();
                    let color = it.next().unwrap();

                    match color {
                        "red" => red = red.max(num),
                        "green" => green = green.max(num),
                        "blue" => blue = blue.max(num),
                        _ => panic!("invalid color"),
                    }
                })
            });

            red * green * blue
        })
        .sum();

    Ok(res.to_string())
}

#[test]
fn test_part1() {
    let input: String = String::from(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    );

    assert_eq!(String::from("8"), part1(input).unwrap())
}

#[test]
fn test_part2() {
    let input: String = String::from(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    );

    assert_eq!(String::from("2286"), part2(input).unwrap())
}
