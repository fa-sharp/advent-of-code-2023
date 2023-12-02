use fancy_regex::Regex;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut values: Vec<u32> = vec![];
    input.lines().for_each(|line| {
        let mut line_values: Vec<u32> = vec![];
        line.chars().for_each(|c| {
            if c.is_ascii_digit() {
                if line_values.len() < 2 {
                    line_values.push(c.to_digit(10).expect("Error parsing digit!"));
                } else {
                    line_values.pop();
                    line_values.push(c.to_digit(10).expect("Error parsing digit!"));
                }
            }
        });
        if line_values.len() == 2 {
            values.push(
                line_values.get(0).expect("No digit found at index 0!") * 10
                    + line_values.get(1).expect("No digit found at index 1!"),
            );
        } else if line_values.len() == 1 {
            values.push(line_values.get(0).expect("No digit found at index 0!") * 11);
        }
    });

    let sum = values.into_iter().fold(0, |sum, val| sum + val);

    Some(sum)
}

fn parse_spelled_digit(digit: &str) -> Option<u32> {
    match digit {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    // magic, probably unsafe regex that detects both string and number digits
    let digit_regex = Regex::new(r"(?=(one|two|three|four|five|six|seven|eight|nine|\d))").unwrap();

    let mut values: Vec<u32> = vec![];
    input.lines().for_each(|line| {
        // Find all matches in this line for digits (either string or number) using the magic regex
        let mut line_matches = digit_regex.captures_iter(line).map(|capture| {
            capture
                .unwrap()
                .iter()
                .last()
                .unwrap()
                .map(|m| m.range())
                .expect("Error getting range of a match")
        });
        // Get the first and last match
        let first_match = line_matches.next().expect("Couldn't get first match");
        let last_match = line_matches.last().unwrap_or_else(|| first_match.clone());

        // Calculate the actual value
        let mut value: u32 = 0;

        let raw_first_digit = line
            .get(first_match)
            .expect("Couldn't read raw first digit");
        if let Ok(first_digit) = raw_first_digit.parse::<u32>() {
            value += 10 * first_digit;
        } else {
            value += 10
                * parse_spelled_digit(raw_first_digit)
                    .expect("Couldn't parse first digit as either number or string");
        }

        let raw_second_digit = line.get(last_match).expect("Couldn't read raw last digit");
        if let Ok(second_digit) = raw_second_digit.parse::<u32>() {
            value += second_digit;
        } else {
            value += parse_spelled_digit(raw_second_digit)
                .expect("Couldn't parse last digit as either number or string");
        }

        values.push(value);
    });

    let sum = values.into_iter().fold(0, |sum, val| sum + val);

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
