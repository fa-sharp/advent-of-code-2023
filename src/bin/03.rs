advent_of_code::solution!(3);

#[derive(PartialEq, Eq)]
enum Cell {
    Digit(u8),
    Symbol(char),
    Empty,
}
impl Cell {
    fn parse(char: char) -> Cell {
        if char.is_digit(10) {
            return Cell::Digit(
                u8::try_from(char.to_digit(10).expect("Couldn't parse digit!")).unwrap(),
            );
        } else if char != '.' && char.is_ascii_punctuation() {
            return Cell::Symbol(char);
        } else {
            return Cell::Empty;
        }
    }
}

fn parse_input(input: &str) -> Vec<((usize, usize), Cell)> {
    let mut grid: Vec<((usize, usize), Cell)> = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            grid.push(((x, y), Cell::parse(char)));
        }
    }
    return grid;
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let line_length = input.lines().next().unwrap().len();
    let mut part_numbers: Vec<u32> = vec![];

    for line in grid.chunks(line_length) {
        let mut current_num: u32 = 0;
        let mut is_part_num = false;

        for ((x, y), cell) in line {
            match cell {
                Cell::Digit(digit) => {
                    current_num *= 10;
                    current_num += u32::from(*digit);
                    // find out if any adjacent cells are symbols, and if so, flag as a part number
                    for adj_coord in get_adj_coords((*x, *y)) {
                        if let Some((_, adj_cell)) =
                            grid.iter().find(|(coord, _)| adj_coord == *coord)
                        {
                            if matches!(adj_cell, Cell::Symbol(_)) {
                                is_part_num = true;
                            }
                        }
                    }
                }
                Cell::Symbol(_) | Cell::Empty => {
                    if is_part_num && current_num > 0 {
                        part_numbers.push(current_num);
                    }
                    current_num = 0;
                    is_part_num = false;
                }
            }
        }
        if is_part_num && current_num > 0 {
            part_numbers.push(current_num);
        }
    }

    Some(part_numbers.iter().fold(0, |sum, n| sum + n))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn get_adj_coords((x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let x_range = if x > 0 { x - 1..x + 2 } else { 0..2 };
    let y_range = if y > 0 { y - 1..y + 2 } else { 0..2 };
    x_range
        .flat_map(move |x| y_range.clone().map(move |y| (x, y)))
        .filter(move |&q| (x, y) != q)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
