advent_of_code::solution!(2);

enum Color {
    Red,
    Green,
    Blue,
}

struct CubeSet {
    blue: u32,
    red: u32,
    green: u32,
}

pub fn part_one(input: &str) -> Option<usize> {
    let games: Vec<Vec<CubeSet>> = parse_input(input).expect("Issue while parsing input!");

    // Iterate through games, and filter only the ones that are possible
    let possible_games = games.iter().enumerate().filter(|(_, game_sets)| {
        game_sets
            .iter()
            .all(|set| set.blue <= 14 && set.red <= 12 && set.green <= 13)
    });

    // Calculate sum of game indices
    let sum_game_indices = possible_games.fold(0, |sum, (game_idx, _)| sum + game_idx + 1); // +1 cuz it starts at Game 1 😅

    Some(sum_game_indices)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games: Vec<Vec<CubeSet>> = parse_input(input).expect("Issue while parsing input!");

    // Iterate through games, and for each one...
    let sum_powers = games
        .iter()
        // calculate the minimum possible # of cubes per color
        .map(|game_sets| CubeSet {
            blue: game_sets.iter().max_by_key(|set| set.blue).unwrap().blue,
            red: game_sets.iter().max_by_key(|set| set.red).unwrap().red,
            green: game_sets.iter().max_by_key(|set| set.green).unwrap().green,
        })
        // calculate sum of powers
        .fold(0, |sum, minimums| {
            sum + (minimums.blue * minimums.green * minimums.red)
        });

    Some(sum_powers)
}

/** Turn the input into a workable array of game results */
fn parse_input(input: &str) -> Option<Vec<Vec<CubeSet>>> {
    let mut games: Vec<Vec<CubeSet>> = vec![];

    for line in input.lines() {
        let (_, raw_sets) = line.split_once(": ").unwrap();

        let mut sets: Vec<CubeSet> = vec![];

        for raw_set in raw_sets.split("; ") {
            let mut set: CubeSet = CubeSet {
                blue: 0,
                red: 0,
                green: 0,
            };
            for num_and_color in raw_set.split(", ") {
                let (raw_num, raw_color) = num_and_color.split_once(" ")?;
                let num: u32 = raw_num.parse().ok()?;
                let color = parse_color(raw_color)?;
                match color {
                    Color::Red => set.red += num,
                    Color::Green => set.green += num,
                    Color::Blue => set.blue += num,
                }
            }
            sets.push(set);
        }
        games.push(sets);
    }

    Some(games)
}

fn parse_color(raw_color: &str) -> Option<Color> {
    match raw_color {
        "blue" => Some(Color::Blue),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
