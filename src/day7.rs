use once_cell::sync::Lazy;
use regex::Regex;

pub fn part1(bags: Bags) -> usize {
    let mut can_contain = 0;
    for bag in &bags.0 {
        if bag.can_contain("shiny gold", &bags) {
            can_contain += 1;
        }
    }
    can_contain
}

pub fn part2(bags: Bags) -> i32 {
    let shiny_gold = bags.find_color("shiny gold").unwrap();
    shiny_gold.count_content(&bags, 1)
}

static REGEX_CONTAINS: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d+)\s(.+)$").unwrap());

#[derive(Debug, PartialEq)]
pub struct Bags(Vec<Bag>);

impl Bags {
    pub fn find_color(&self, color: &str) -> Option<&Bag> {
        self.0.iter().find(|b| b.color == color)
    }
}

impl From<&str> for Bags {
    fn from(s: &str) -> Self {
        Bags(s.lines().map(Into::into).collect())
    }
}

#[derive(Debug, PartialEq)]
pub struct Bag {
    color: String,
    contain: Contains,
}

impl Bag {
    fn all_colors<'a>(&'a self, bags: &'a Bags) -> Vec<&'a str> {
        self.contain
            .0
            .iter()
            .map(|(_, color)| color.as_str())
            .flat_map(|c| {
                let mut output_colors = vec![c];
                output_colors.append(
                    &mut bags
                        .find_color(c)
                        .map(|b| b.all_colors(bags))
                        .unwrap_or_default(),
                );
                output_colors
            })
            .collect()
    }

    pub fn can_contain(&self, color: &str, bags: &Bags) -> bool {
        self.all_colors(bags).contains(&color)
    }

    pub fn count_content(&self, bags: &Bags, multiplier: i32) -> i32 {
        self.contain.0.iter().fold(0, |acc, (count, color)| {
            let bag = bags.find_color(color);
            let child_count = bag
                .map(|b| b.count_content(&bags, *count * multiplier))
                .unwrap_or_default();
            acc + (count * multiplier) + child_count
        })
    }
}

impl From<&str> for Bag {
    fn from(s: &str) -> Self {
        let contains = s.split("bags contain").collect::<Vec<&str>>();
        match contains.as_slice() {
            [color, contain] => Self {
                color: color.trim().to_string(),
                contain: (*contain).into(),
            },
            _ => panic!("invalid input"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Contains(Vec<(i32, String)>);

impl From<&str> for Contains {
    fn from(s: &str) -> Self {
        Self(
            s.split(',')
                .collect::<Vec<&str>>()
                .iter()
                .map(|c| {
                    c.trim_end_matches('.')
                        .trim_end_matches("bag")
                        .trim_end_matches("bags")
                        .trim()
                })
                .flat_map(|c| {
                    if c == "no other" {
                        return vec![];
                    }
                    let captures = REGEX_CONTAINS.captures(c).unwrap();
                    vec![(
                        captures.get(1).unwrap().as_str().parse().unwrap(),
                        captures.get(2).unwrap().as_str().to_string(),
                    )]
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn parse_line() {
        let s = "light red bags contain 1 bright white bag, 2 muted yellow bags.";

        let bag: Bag = s.into();
        assert_eq!(
            Bag {
                color: "light red".to_string(),
                contain: Contains(vec![
                    (1, "bright white".to_owned()),
                    (2, "muted yellow".to_owned())
                ])
            },
            bag
        )
    }

    #[test]
    pub fn parse_line2() {
        let s = "bright white bags contain 1 shiny gold bag.";

        let bag: Bag = s.into();
        assert_eq!(
            Bag {
                color: "bright white".to_string(),
                contain: Contains(vec![(1, "shiny gold".to_owned())])
            },
            bag
        )
    }

    #[test]
    pub fn all_containers() {
        let s = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let bags: Bags = s.into();
        assert_eq!(4, part1(bags));
    }

    #[test]
    pub fn count_content() {
        let s = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        let bags: Bags = s.into();
        assert_eq!(126, part2(bags));
    }
}
