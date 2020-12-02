use std::ops::RangeInclusive;
use std::str::FromStr;

pub fn first_step(entries: Passwords<PasswordDay1>) -> i32 {
    entries.valid_ones().len() as i32
}

pub fn second_step(entries: Passwords<PasswordDay2>) -> i32 {
    entries.valid_ones().len() as i32
}

pub trait ValidatedPassword {
    fn is_valid(&self) -> bool;
}

#[derive(Debug)]
pub struct Passwords<T>(Vec<T>);

impl<T: ValidatedPassword> Passwords<T> {
    pub fn valid_ones(&self) -> Vec<&T> {
        self.0.iter().filter(|e| e.is_valid()).collect()
    }
}

impl<T: FromStr> FromStr for Passwords<T>
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines().map(|line| line.parse::<T>().unwrap()).collect(),
        ))
    }
}

#[derive(PartialEq, Debug)]
pub struct PasswordDay1 {
    password: String,
    occurrences: RangeInclusive<i32>,
    char: char,
}

impl ValidatedPassword for PasswordDay1 {
    fn is_valid(&self) -> bool {
        let num_char = self.password.chars().filter(|c| c == &self.char).count();
        self.occurrences.contains(&(num_char as i32))
    }
}

impl FromStr for PasswordDay1 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(':').collect::<Vec<&str>>().as_slice() {
            [policy_pattern, password] => {
                match policy_pattern.split(' ').collect::<Vec<&str>>().as_slice() {
                    [range, char] => {
                        let range_values = range
                            .split('-')
                            .map(|v| v.parse::<i32>().unwrap())
                            .collect::<Vec<i32>>();

                        Ok(Self {
                            password: password.trim().to_string(),
                            occurrences: RangeInclusive::new(range_values[0], range_values[1]),
                            char: char.chars().next().unwrap(),
                        })
                    }
                    _ => Err(format!("no policy could be extracted from {}", s)),
                }
            }
            _ => Err(format!("the line {} is not valid", s)),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct PasswordDay2 {
    password: String,
    positions: (i32, i32),
    char: char,
}

impl ValidatedPassword for PasswordDay2 {
    fn is_valid(&self) -> bool {
        let (pos1, pos2) = self.positions;
        let password_chars: Vec<char> = self.password.chars().collect();
        let chars = vec![
            password_chars[(pos1 - 1) as usize],
            password_chars[(pos2 - 1) as usize],
        ];
        chars
            .iter()
            .fold(false, |acc, c| if c == &self.char { !acc } else { acc })
    }
}

impl FromStr for PasswordDay2 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(':').collect::<Vec<&str>>().as_slice() {
            [policy_pattern, password] => {
                match policy_pattern.split(' ').collect::<Vec<&str>>().as_slice() {
                    [range, char] => {
                        let range_values = range
                            .split('-')
                            .map(|v| v.parse::<i32>().unwrap())
                            .collect::<Vec<i32>>();

                        Ok(Self {
                            password: password.trim().to_string(),
                            positions: (range_values[0], range_values[1]),
                            char: char.chars().next().unwrap(),
                        })
                    }
                    _ => Err(format!("no policy could be extracted from {}", s)),
                }
            }
            _ => Err(format!("the line {} is not valid", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn type_test() {
        let input = "1-3 a: abcde";
        let password_day_1 = PasswordDay1 {
            password: "abcde".to_owned(),
            occurrences: 1..=3,
            char: 'a',
        };

        assert_eq!(password_day_1, input.parse().unwrap());
    }

    #[test]
    pub fn input_test_1() {
        let input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

        let entries: Passwords<PasswordDay1> = input.parse().unwrap();
        assert_eq!(2, entries.valid_ones().len());
    }

    #[test]
    pub fn input_test_2() {
        let input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccccc";

        let entries: Passwords<PasswordDay1> = input.parse().unwrap();
        assert_eq!(1, entries.valid_ones().len());
    }

    #[test]
    pub fn input_test_3() {
        let input = "1-3 a: abcde
1-3 e: cdefg
2-9 c: ccccccccc";

        let entries: Passwords<PasswordDay1> = input.parse().unwrap();
        assert_eq!(3, entries.valid_ones().len());
    }

    #[test]
    pub fn input_test_1_new_password_policy() {
        let input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

        let entries: Passwords<PasswordDay2> = input.parse().unwrap();
        assert_eq!(1, entries.valid_ones().len());
    }
}
