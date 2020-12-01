use crate::utils::*;
use std::str::FromStr;

#[derive(Debug)]
pub struct Numbers(Vec<i32>);

pub fn one(data: Numbers) {
    let permutations = compute_permutations(data.0);
    let correct_permutation = permutations
        .iter()
        .find(|(first, second)| first + second == 2020)
        .unwrap();
    print_result(correct_permutation.0 * correct_permutation.1);
}

pub fn two(data: Numbers) {
    let permutations = compute_permutations_for_3(data.0);
    let correct_permutation = permutations
        .iter()
        .find(|(first, second, third)| first + second + third == 2020)
        .unwrap();
    print_result(correct_permutation.0 * correct_permutation.1 * correct_permutation.2);
}

fn compute_permutations<T: Copy>(list: Vec<T>) -> Vec<(T, T)> {
    let mut result = vec![];
    for (i, first) in list.iter().enumerate() {
        for second in list[i + 1..].iter() {
            result.push((*first, *second))
        }
    }
    result
}

fn compute_permutations_for_3<T: Copy>(list: Vec<T>) -> Vec<(T, T, T)> {
    let mut result = vec![];
    for (i, first) in list.iter().enumerate() {
        for second in list[i + 1..].iter() {
            for third in list[i + 2..].iter() {
                result.push((*first, *second, *third))
            }
        }
    }
    result
}

impl FromStr for Numbers {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s.lines().map(|n| n.parse().unwrap()).collect();

        Ok(Numbers(numbers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_permutations_for_3_element_list() {
        let list: Vec<i32> = vec![1, 2, 3];
        assert_eq!(vec![(1, 2), (1, 3), (2, 3)], compute_permutations(list));
    }

    #[test]
    fn compute_permutations_for_empty_list() {
        let list: Vec<i32> = vec![];
        let result: Vec<(i32, i32)> = vec![];

        assert_eq!(result, compute_permutations(list));
    }

    #[test]
    fn compute_permutations_for_single_element_list() {
        let list: Vec<i32> = vec![1];
        let result: Vec<(i32, i32)> = vec![];

        assert_eq!(result, compute_permutations(list));
    }
}
