use std::collections::HashSet;

use itertools::Itertools;
use nom::Slice;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_points: u32 = 0;

    for line in input.lines() {
        let Some((raw_winning_nums, raw_card_nums)) = line
            .slice((line.find(':').unwrap() + 1)..)
            .split_once(" | ")
        else {
            panic!("Error dividing input line!")
        };

        let mut winning_nums: HashSet<u32> = HashSet::new();
        for winning_num in &raw_winning_nums.chars().chunks(3) {
            winning_nums.insert(
                winning_num
                    .filter(|c| c.is_numeric())
                    .collect::<String>()
                    .parse()
                    .expect("Error parsing winning number!"),
            );
        }

        let mut num_card_winning_nums: u32 = 0;
        for card_num in &raw_card_nums.chars().chunks(3) {
            if winning_nums.contains(
                &card_num
                    .filter(|c| c.is_numeric())
                    .collect::<String>()
                    .parse()
                    .expect("Error parsing card number!"),
            ) {
                num_card_winning_nums += 1;
            }
        }
        if num_card_winning_nums > 0 {
            total_points += 2_u32.pow(num_card_winning_nums - 1)
        }
    }

    Some(total_points)
}

pub fn part_two(input: &str) -> Option<u32> {
    let total_lines = input.lines().count();
    let mut all_scratchcards: Vec<u32> = vec![1; total_lines];

    for (line_idx, line) in input.lines().enumerate() {
        let Some((raw_winning_nums, raw_card_nums)) = line
            .slice((line.find(':').unwrap() + 1)..)
            .split_once(" | ")
        else {
            panic!("Error dividing input line!")
        };

        let mut winning_nums: HashSet<u32> = HashSet::new();
        for winning_num in &raw_winning_nums.chars().chunks(3) {
            winning_nums.insert(
                winning_num
                    .filter(|c| c.is_numeric())
                    .collect::<String>()
                    .parse()
                    .expect("Error parsing winning number!"),
            );
        }

        let num_scratchcards_at_this_idx = all_scratchcards[line_idx];

        let mut num_winning_nums: u32 = 0;
        for card_num in &raw_card_nums.chars().chunks(3) {
            if winning_nums.contains(
                &card_num
                    .filter(|c| c.is_numeric())
                    .collect::<String>()
                    .parse()
                    .expect("Error parsing card number!"),
            ) {
                let scratchcard_idx = line_idx + 1 + (num_winning_nums as usize);
                if scratchcard_idx < total_lines {
                    all_scratchcards[scratchcard_idx] += num_scratchcards_at_this_idx;
                    num_winning_nums += 1;
                }
            }
        }
    }

    Some(all_scratchcards.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result: Option<u32> = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
