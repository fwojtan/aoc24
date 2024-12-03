use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1};
use nom::error::{ErrorKind, ParseError};
use nom::multi::many1;
use nom::sequence::tuple;
use nom::{Finish, IResult};

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day03;

impl Solution for Day03 {
    type ParsedInput = Vec<Instruction>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        parser(input_lines).finish().expect("Should parse ok").1
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        parsed_input.iter().filter_map(|i| {
            match i {
                Instruction::Mul(a) => Some(a),
                Instruction::Do | Instruction::Dont => None,
            }
        }).sum::<u64>().to_string()
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        let mut enabled = true;
        let mut total = 0;
        for instruction in parsed_input {
            match instruction {
                Instruction::Mul(a) => {
                    if enabled {
                        total += *a;
                    }
                },
                Instruction::Do => enabled = true,
                Instruction::Dont => enabled = false,
            }
        }
        total.to_string()
    }
}

// fn parse_mul(i: &str) -> IResult<&str, (u64, u64)> {
//     // this surely can't be very fast because it's allocating a new vec for the stuff we're skipping
//     many_till(anychar, tuple((tag("mul("), digit1, char(','), digit1, char(')'))))(i).map(|(rem, (_v, (_a, b, _c, d, _e)))| {
//         (rem, (b.parse::<u64>().unwrap(), d.parse::<u64>().unwrap()))
//     })
// }

#[derive(Debug)]
pub enum Instruction {
    Mul(u64),
    Do,
    Dont,
}

fn parse_mul<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Instruction> {
    tuple((tag("mul("), digit1::<&str, E>, char(','), digit1, char(')')))(i)
    .map(|(rem, (_a, b, _c, d, _e))| (rem, Instruction::Mul(b.parse::<u64>().unwrap() * d.parse::<u64>().unwrap())))
    .map_err(|e| {match e {
        nom::Err::Incomplete(needed) => nom::Err::Incomplete(needed),
        nom::Err::Error(_) => nom::Err::Error(nom::error::Error::new(i, ErrorKind::Digit)),
        nom::Err::Failure(_) => nom::Err::Failure(nom::error::Error::new(i, ErrorKind::Fail)),
    }})
}

fn parse_do<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Instruction> {
    tag("do()")(i).map(|(rem, _val)| (rem, Instruction::Do))
}

fn parse_dont<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Instruction> {
    tag("don't()")(i).map(|(rem, _val)| (rem, Instruction::Dont))
}

fn parse_next_instruction<'a, E: ParseError<&'a str>>(mut i: &'a str) -> IResult<&'a str, Instruction> {
    loop {
        match alt((parse_mul::<E>, parse_do::<E>, parse_dont::<E>))(i) {
            Ok((rem, instruction)) => return Ok((rem, instruction)),
            Err(nom::Err::Error(_e)) => if !i.is_empty() {i = &i[1..]} else {return Err(nom::Err::Error(nom::error::Error::new(i, ErrorKind::Complete)))},
            Err(nom::Err::Failure(_e)) => return Err(nom::Err::Failure(nom::error::Error::new(i, ErrorKind::Fail))),
            Err(nom::Err::Incomplete(e)) => return Err(nom::Err::Incomplete(e)),
        }
    }
}

fn parser<'a>(i: &'a str) -> IResult<&'a str, Vec<Instruction>> {
    many1(parse_next_instruction::<nom::error::Error<&'a str>>)(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day03_part1_case1() {
        // println!("{:?}", parser("mul(100,100)haufmul(haanfkolamul(1,0)"));
        assert_eq!(Day03::solve_part_one(""), "100".to_string())
    }

    #[test]
    fn check_day03_part2_case1() {
        assert_eq!(Day03::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day03_both_case1() {
        assert_eq!(Day03::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
