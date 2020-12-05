use std::iter::Peekable;
use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct Positions<'a>(Vec<&'a str>);

impl<'a> From<&'a str> for Positions<'a> {
    fn from(s: &'a str) -> Self {
        Self(s.lines().collect())
    }
}

pub fn part1(positions: Positions) -> u32 {
    positions
        .0
        .iter()
        .map(|pos| calculate_id(pos))
        .max()
        .unwrap()
}

pub fn part2(positions: Positions) -> u32 {
    let mut ids = positions
        .0
        .iter()
        .map(|pos| calculate_id(pos))
        .collect::<Vec<u32>>();
    ids.sort();
    let mut iterator = ids.iter().peekable();
    find_non_consecutive_numbers(&mut iterator)
}

fn find_non_consecutive_numbers(iterator: &mut Peekable<std::slice::Iter<u32>>) -> u32 {
    let actual = iterator.next().unwrap();
    let next = iterator.peek().unwrap();

    if *actual + 1 != **next {
        return *actual + 1;
    }
    find_non_consecutive_numbers(iterator)
}

fn calculate_row(pos: &str) -> u32 {
    let (row, _) = pos.split_at(7);
    do_calculate_range(row, 0..=127, 'F')
}
fn calculate_col(pos: &str) -> u32 {
    let (_, seat) = pos.split_at(7);
    do_calculate_range(seat, 0..=7, 'L')
}

fn do_calculate_range(chars: &str, range: RangeInclusive<u32>, lower_char: char) -> u32 {
    let (_, final_val) = chars.chars().fold((range, 0), |(seat, _), char| {
        let diff = seat.end() - seat.start();
        if diff == 1 {
            return (
                0..=0,
                *if char == lower_char {
                    seat.start()
                } else {
                    seat.end()
                },
            );
        }
        let new = if char == lower_char {
            RangeInclusive::new(*seat.start(), seat.start() + diff / 2)
        } else {
            RangeInclusive::new(seat.end() - diff / 2, *seat.end())
        };
        (new, 0)
    });
    final_val
}

fn calculate_id(pos: &str) -> u32 {
    calculate_row(pos) * 8 + calculate_col(pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn pos_1() {
        let pos = "BFFFBBFRRR";
        assert_eq!(70, calculate_row(pos));
        assert_eq!(7, calculate_col(pos));
        assert_eq!(567, calculate_id(pos));
    }

    #[test]
    pub fn pos_2() {
        let pos = "FFFBBBFRRR";
        assert_eq!(14, calculate_row(pos));
        assert_eq!(7, calculate_col(pos));
        assert_eq!(119, calculate_id(pos));
    }

    #[test]
    pub fn pos_3() {
        let pos = "BBFFBBFRLL";
        assert_eq!(102, calculate_row(pos));
        assert_eq!(4, calculate_col(pos));
        assert_eq!(820, calculate_id(pos));
    }
}
