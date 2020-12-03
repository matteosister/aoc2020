use std::str::FromStr;

#[derive(Debug)]
pub struct Lines(Vec<String>);

impl FromStr for Lines {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.lines().map(ToString::to_string).collect()))
    }
}

fn traverse_slope(lines: &[String], right: usize, down: usize) -> i64 {
    let (trees, _) = lines
        .iter()
        .step_by(down)
        .fold((0, 0), |(trees, line_p), row| {
            let element = row.chars().cycle().nth(line_p);
            (
                trees + (element.map(|el| if el == '#' { 1 } else { 0 }).unwrap_or(0)),
                line_p + right,
            )
        });
    trees
}

pub fn part1(lines: Lines) -> i64 {
    traverse_slope(lines.0.as_slice(), 3, 1)
}

pub fn part2(lines: Lines) -> i64 {
    let run1 = traverse_slope(lines.0.as_slice(), 1, 1);
    let run2 = traverse_slope(lines.0.as_slice(), 3, 1);
    let run3 = traverse_slope(lines.0.as_slice(), 5, 1);
    let run4 = traverse_slope(lines.0.as_slice(), 7, 1);
    let run5 = traverse_slope(lines.0.as_slice(), 1, 2);

    run1 * run2 * run3 * run4 * run5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run1() {
        let lines: Lines = input().parse().unwrap();

        assert_eq!(2, traverse_slope(lines.0.as_slice(), 1, 1));
    }

    #[test]
    fn run2() {
        let lines: Lines = input().parse().unwrap();

        assert_eq!(7, traverse_slope(lines.0.as_slice(), 3, 1));
    }

    #[test]
    fn run3() {
        let lines: Lines = input().parse().unwrap();

        assert_eq!(3, traverse_slope(lines.0.as_slice(), 5, 1));
    }

    #[test]
    fn run4() {
        let lines: Lines = input().parse().unwrap();

        assert_eq!(4, traverse_slope(lines.0.as_slice(), 7, 1));
    }

    #[test]
    fn run5() {
        let lines: Lines = input().parse().unwrap();

        assert_eq!(2, traverse_slope(lines.0.as_slice(), 1, 2));
    }

    #[test]
    fn product() {
        let lines: Lines = input().parse().unwrap();

        assert_eq!(
            336,
            traverse_slope(lines.0.as_slice(), 1, 1)
                * traverse_slope(lines.0.as_slice(), 3, 1)
                * traverse_slope(lines.0.as_slice(), 5, 1)
                * traverse_slope(lines.0.as_slice(), 7, 1)
                * traverse_slope(lines.0.as_slice(), 1, 2)
        );
    }

    fn input() -> &'static str {
        "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
    }
}
