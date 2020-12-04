use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::ops::RangeBounds;

static REGEX_COLOR: Lazy<Regex> = Lazy::new(|| Regex::new(r"^#[a-f0-9]{6}$").unwrap());
static REGEX_PID: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d{9}$").unwrap());
static REGEX_HEIGHT: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d+)(cm|in)$").unwrap());

pub fn part1(passports: Passports<PassportData>) -> u32 {
    passports.0.iter().filter(|p| p.is_valid()).count() as u32
}

pub fn part2(passports: Passports<PassportDataStrict>) -> u32 {
    passports.0.iter().filter(|p| p.is_valid()).count() as u32
}

#[derive(Debug)]
pub struct Passports<T>(Vec<T>);

impl<'a> From<&'a str> for Passports<PassportData<'a>> {
    fn from(s: &'a str) -> Self {
        Self(s.split("\n\n").map(Into::into).collect())
    }
}

impl<'a> From<&'a str> for Passports<PassportDataStrict<'a>> {
    fn from(s: &'a str) -> Self {
        Self(s.split("\n\n").map(Into::into).collect())
    }
}

#[derive(Debug, PartialEq)]
pub struct PassportData<'a>(HashMap<&'a str, &'a str>);

impl<'a> PassportData<'a> {
    pub fn is_valid(&self) -> bool {
        self.0.get("byr").is_some()
            && self.0.get("iyr").is_some()
            && self.0.get("eyr").is_some()
            && self.0.get("hgt").is_some()
            && self.0.get("hcl").is_some()
            && self.0.get("ecl").is_some()
            && self.0.get("pid").is_some()
    }
}

impl<'a> From<&'a str> for PassportData<'a> {
    fn from(s: &'a str) -> Self {
        let fields: HashMap<&str, &str> = s
            .split_whitespace()
            .map(|data| {
                let parts: Vec<&str> = data.split(':').collect();
                (parts[0], parts[1])
            })
            .collect();

        Self(fields)
    }
}

#[derive(Debug, PartialEq)]
pub struct PassportDataStrict<'a>(HashMap<&'a str, &'a str>);

impl<'a> PassportDataStrict<'a> {
    pub fn is_valid(&self) -> bool {
        let valid_byr = self
            .0
            .get("byr")
            .map(|y| Self::validate_num(y, 1920..=2002))
            .unwrap_or_default();

        let valid_iyr = self
            .0
            .get("iyr")
            .map(|y| Self::validate_num(y, 2010..=2020))
            .unwrap_or_default();

        let valid_eyr = self
            .0
            .get("eyr")
            .map(|y| Self::validate_num(y, 2020..=2030))
            .unwrap_or_default();

        let valid_hgt = self
            .0
            .get("hgt")
            .map(|h| Self::validate_height(h))
            .unwrap_or_default();

        let valid_hcl = self
            .0
            .get("hcl")
            .map(|hcl| REGEX_COLOR.is_match(hcl))
            .unwrap_or_default();

        let valid_ecl = self
            .0
            .get("ecl")
            .map(|ecl| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(ecl))
            .unwrap_or_default();

        let valid_pid = self
            .0
            .get("pid")
            .map(|pid| REGEX_PID.is_match(pid))
            .unwrap_or_default();

        valid_byr && valid_iyr && valid_eyr && valid_hgt && valid_hcl && valid_ecl && valid_pid
    }

    fn validate_num(num: &str, range: impl RangeBounds<i32>) -> bool {
        num.parse::<i32>()
            .ok()
            .map(|y| range.contains(&y))
            .unwrap_or_default()
    }

    fn validate_height(s: &str) -> bool {
        let height: Result<Height, ()> = s.try_into();
        match height {
            Ok(h) => h.is_valid(),
            _ => false,
        }
    }
}

#[derive(Debug)]
enum Height {
    Cm(i32),
    In(i32),
}

impl Height {
    pub fn is_valid(&self) -> bool {
        match self {
            Height::Cm(v) => (150..=193).contains(v),
            Height::In(v) => (59..=76).contains(v),
        }
    }
}

impl<'a> TryFrom<&'a str> for Height {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let caps = REGEX_HEIGHT.captures(value);
        caps.map(|c| {
            if c.get(2).unwrap().as_str() == "cm" {
                Ok(Self::Cm(c.get(1).unwrap().as_str().parse().unwrap()))
            } else {
                Ok(Self::In(c.get(1).unwrap().as_str().parse().unwrap()))
            }
        })
        .unwrap_or_else(|| Err(()))
    }
}

impl<'a> From<&'a str> for PassportDataStrict<'a> {
    fn from(s: &'a str) -> Self {
        let fields: HashMap<&str, &str> = s
            .split_whitespace()
            .map(|data| {
                let parts: Vec<&str> = data.split(':').collect();
                (parts[0], parts[1])
            })
            .collect();

        Self(fields)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn parse_data_1() {
        let data = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm";
        let passport_data: PassportData = data.into();

        assert!(passport_data.is_valid());
    }

    #[test]
    pub fn parse_data_2() {
        let data = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929";
        let passport_data: PassportData = data.into();

        assert!(!passport_data.is_valid());
    }

    #[test]
    pub fn test_invalid_passport_1() {
        let s = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926";
        let pd: PassportDataStrict = s.into();

        assert!(!pd.is_valid());
    }

    #[test]
    pub fn test_invalid_passport_2() {
        let s = "iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946";
        let pd: PassportDataStrict = s.into();

        assert!(!pd.is_valid());
    }

    #[test]
    pub fn test_invalid_passport_3() {
        let s = "hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277";
        let pd: PassportDataStrict = s.into();

        assert!(!pd.is_valid());
    }

    #[test]
    pub fn test_invalid_passport_4() {
        let s = "hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        let pd: PassportDataStrict = s.into();

        assert!(!pd.is_valid());
    }

    #[test]
    pub fn test_valid_passport_1() {
        let s = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f";
        let pd: PassportDataStrict = s.into();

        assert!(pd.is_valid());
    }

    #[test]
    pub fn test_valid_passport_2() {
        let s = "eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm";
        let pd: PassportDataStrict = s.into();

        assert!(pd.is_valid());
    }

    #[test]
    pub fn test_valid_passport_3() {
        let s = "hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022";
        let pd: PassportDataStrict = s.into();

        assert!(pd.is_valid());
    }

    #[test]
    pub fn test_valid_passport_4() {
        let s = "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let pd: PassportDataStrict = s.into();

        assert!(pd.is_valid());
    }

    #[test]
    pub fn test_valid_passport_5() {
        let s = "iyr:2010 hgt:193cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let pd: PassportDataStrict = s.into();

        assert!(pd.is_valid());
    }

    #[test]
    pub fn test_valid_passport_6() {
        let s = "iyr:2010 hgt:59in hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let pd: PassportDataStrict = s.into();

        assert!(pd.is_valid());
    }
}
