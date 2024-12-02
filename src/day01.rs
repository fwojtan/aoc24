use counter::Counter;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day01;

impl Solution for Day01 {
    type ParsedInput = (Vec<u32>, Vec<u32>);

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        let n_lines = input_lines.lines().count();
        let mut first_vec = Vec::with_capacity(n_lines);
        let mut second_vec = Vec::with_capacity(n_lines);
        for line in input_lines.lines() {
            let mut split = line.split_ascii_whitespace();
            first_vec.push(split.next().unwrap().parse::<u32>().unwrap());
            second_vec.push(split.next().unwrap().parse::<u32>().unwrap());
        }
        (first_vec, second_vec)
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        parsed_input.0.sort();
        parsed_input.1.sort();
        parsed_input
            .0
            .iter()
            .zip(parsed_input.1.iter())
            .map(|vals| vals.0.abs_diff(*vals.1))
            .sum::<u32>()
            .to_string()
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        let counter = parsed_input.1.iter().collect::<Counter<_>>();
        parsed_input
            .0
            .iter()
            .map(|val| (*val as usize) * *counter.get(val).unwrap_or(&0))
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day01_part1_case1() {
        assert_eq!(Day01::solve_part_one(""), "0".to_string())
    }

    #[test]
    fn check_day01_part2_case1() {
        assert_eq!(Day01::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day01_both_case1() {
        assert_eq!(Day01::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
