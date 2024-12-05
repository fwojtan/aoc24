use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day02;

impl Solution for Day02 {
    type ParsedInput = Vec<Vec<i8>>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        input_lines
            .lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|num| num.parse::<i8>().unwrap())
                    .map_windows(|[a, b]| *b - *a)
                    .collect::<Vec<i8>>()
            })
            .collect::<Vec<Vec<i8>>>()
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        parsed_input
            .iter()
            .map(check_safe)
            .filter(|t| *t)
            .count()
            .to_string()
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        parsed_input
            .iter()
            .map(can_make_safe)
            .filter(|t| *t)
            .count()
            .to_string()
    }
}

fn check_safe(diffs: &Vec<i8>) -> bool {
    let mut is_safe = true;
    let increasing = diffs[0] > 0;
    for diff in diffs {
        if (increasing && *diff < 0)
            || (!increasing && *diff > 0)
            || diff.abs() > 3
            || diff.abs() == 0
        {
            is_safe = false;
            break;
        }
    }
    is_safe
}

fn first_unsafe_diff_idx(diffs: &Vec<i8>) -> Option<usize> {
    let increasing = diffs.iter().filter(|diff| **diff > 0).count() > diffs.len() / 2;
    for (i, diff) in diffs.iter().enumerate() {
        if (increasing && *diff < 0)
            || (!increasing && *diff > 0)
            || diff.abs() > 3
            || diff.abs() == 0
        {
            return Some(i);
        }
    }
    None
}

// fn can_make_safe_brute(diffs: &Vec<i8>) -> bool {
//     let is_safe = if !check_safe(diffs) {
//         // check with first item removed
//         let now_safe = check_safe(&diffs.iter().map(|i| *i).skip(1).collect::<Vec<i8>>());
//         if now_safe {
//             true
//         } else {
//             // check with subsequent items removed
//             let mut now_safe = false;
//             for i in 1..diffs.len() {
//                 let mut new_diffs = vec![];
//                 for j in 0..diffs.len() {
//                     if i == j {
//                         new_diffs.push(diffs[i] + diffs[i - 1]);
//                     } else if j != i - 1 {
//                         new_diffs.push(diffs[j]);
//                     }
//                 }
//                 now_safe = check_safe(&new_diffs);
//                 if now_safe {
//                     break;
//                 }
//             }
//             if now_safe {
//                 true
//             } else {
//                 // check with last item removed
//                 check_safe(
//                     &diffs
//                         .iter()
//                         .map(|i| *i)
//                         .take(diffs.len() - 1)
//                         .collect::<Vec<i8>>(),
//                 )
//             }
//         }
//     } else {
//         // row was already safe
//         true
//     };
//     is_safe
// }

fn can_make_safe(diffs: &Vec<i8>) -> bool {
    let idx = first_unsafe_diff_idx(diffs);
    if let Some(idx) = idx {
        // Construct two candidate lists with the preceeding and subsequent elements removed
        let mut remove_first = vec![];
        let mut remove_second = vec![];
        for j in 0..diffs.len() {
            if idx == j {
                if idx > 0 {
                    if let Some(first) = diffs.get(idx - 1) {
                        remove_first.push(diffs[idx] + first);
                    }
                }
                if let Some(second) = diffs.get(idx + 1) {
                    remove_second.push(diffs[idx] + second);
                }
            } else {
                if idx > 0 && j != idx - 1 {
                    remove_first.push(diffs[j]);
                }
                if j != idx + 1 {
                    remove_second.push(diffs[j])
                }
            }
        }
        first_unsafe_diff_idx(&remove_first).is_none()
            || first_unsafe_diff_idx(&remove_second).is_none()
    } else {
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day02_part1_case1() {
        assert_eq!(Day02::solve_part_one(""), "0".to_string())
    }

    #[test]
    fn check_day02_part2_case1() {
        assert_eq!(
            Day02::solve_part_two(
                "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            ),
            "4".to_string()
        )
    }

    #[test]
    fn check_day02_both_case1() {
        assert_eq!(Day02::solve("", false), ("0".to_string(), "0".to_string()))
    }

    #[test]
    fn check_day_02_can_make_safe_case1() {
        assert_eq!(can_make_safe(&vec![1, -1, -3, -1, -3]), true)
    }

    #[test]
    fn check_day_02_can_make_safe_case2() {
        assert_eq!(can_make_safe(&vec![1, -1, -1, -1, -3, -1]), true)
    }

    #[test]
    fn check_day_02_can_make_safe_case3() {
        assert_eq!(can_make_safe(&vec![-2, 2, 2, 1, 3, 2]), true)
    }
}
