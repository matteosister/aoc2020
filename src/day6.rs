#[derive(Debug)]
pub struct Person {
    answers: Vec<char>,
}

impl Person {
    pub fn from_line(s: &str) -> Self {
        Person {
            answers: s.chars().collect(),
        }
    }

    pub fn answers(&self) -> Vec<&char> {
        self.answers.iter().collect()
    }

    pub fn respond_to(&self, char: &char) -> bool {
        self.answers.contains(char)
    }
}

#[derive(Debug)]
pub struct Group {
    pub persons: Vec<Person>,
}

impl Group {
    pub fn from_lines(s: &str) -> Self {
        Self {
            persons: s.lines().map(Person::from_line).collect(),
        }
    }

    pub fn all_unique_answers(&self) -> Vec<&char> {
        let mut all: Vec<&char> = self.persons.iter().flat_map(|p| p.answers()).collect();
        all.sort();
        all.dedup();
        all
    }

    pub fn all_same_answer(&self) -> Vec<&char> {
        let uniques = self.all_unique_answers();
        uniques.iter().fold(vec![], |mut acc, answer| {
            if self.persons.iter().all(|p| p.respond_to(answer)) {
                acc.push(answer);
            }
            acc
        })
    }
}

#[derive(Debug)]
pub struct Groups(Vec<Group>);

impl From<&str> for Groups {
    fn from(s: &str) -> Self {
        Self(s.split("\n\n").map(Group::from_lines).collect())
    }
}

pub fn part1(groups: Groups) -> usize {
    groups.0.iter().map(|g| g.all_unique_answers().len()).sum()
}

pub fn part2(groups: Groups) -> usize {
    groups.0.iter().map(|g| g.all_same_answer().len()).sum()
}

#[cfg(test)]
mod tests {
    use crate::day6::{part1, Groups};

    #[test]
    fn test1() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let groups: Groups = input.into();
        assert_eq!(5, groups.0.len());
        assert_eq!(11, part1(groups));
    }
}
