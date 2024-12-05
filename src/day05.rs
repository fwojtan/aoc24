use std::collections::{HashMap, HashSet};

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day05;

impl Solution for Day05 {
    type ParsedInput = (HashMap<u8, Vec<u8>>, Vec<Vec<u8>>);

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        let mut split = input_lines.split("\n\n");
        let rules = split.next().unwrap();
        let updates = split.next().unwrap();
        let mut preceeding_pages: HashMap<u8, Vec<u8>> = HashMap::new(); // hashing integers, would a different HM be better?
        for rule in rules.lines() {
            let mut split = rule.split("|");
            let first = split.next().unwrap().parse::<u8>().unwrap();
            let second = split.next().unwrap().parse::<u8>().unwrap();
            match preceeding_pages.entry(second) {
                std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                    occupied_entry.get_mut().push(first);
                }
                std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(vec![first]);
                }
            }
        }
        let mut updates_vec = Vec::with_capacity(updates.lines().count());
        for update in updates.lines() {
            let mut update_vec = Vec::with_capacity(25);
            for value in update.split(",") {
                update_vec.push(value.parse::<u8>().unwrap());
            }
            updates_vec.push(update_vec);
        }
        (preceeding_pages, updates_vec)
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        let mut total: u64 = 0;
        for update in parsed_input.1.iter() {
            if update_is_valid(&parsed_input.0, update) {
                total += update[update.len() / 2] as u64
            }
        }
        total.to_string()
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        let mut total: u64 = 0;
        for update in parsed_input.1.iter_mut() {
            if !update_is_valid(&parsed_input.0, update) {
                reorder_update(&parsed_input.0, update);
                total += update[update.len() / 2] as u64
            }
        }
        total.to_string()
    }
}

fn update_is_valid(rules: &HashMap<u8, Vec<u8>>, update: &Vec<u8>) -> bool {
    let mut seen_pages: HashSet<u8> = HashSet::with_capacity(update.len());
    let mut disallowed_pages: HashSet<u8> = HashSet::with_capacity(update.len());
    for page in update {
        if disallowed_pages.contains(page) {
            // print_is_valid(rules, update);
            return false;
        }
        match rules.get(page) {
            Some(preceeding_pages) => {
                for prev_page in preceeding_pages {
                    if !seen_pages.contains(prev_page) {
                        disallowed_pages.insert(*prev_page);
                    } else {
                    }
                }
            }
            None => (),
        }
        seen_pages.insert(*page);
    }
    true
}

fn reorder_update(rules: &HashMap<u8, Vec<u8>>, update: &mut Vec<u8>) {
    loop {
        if let Some((a, b)) = idxs_to_swap(rules, update) {
            update.swap(a, b);
        } else {
            return;
        }
    }
}

fn idxs_to_swap(rules: &HashMap<u8, Vec<u8>>, update: &Vec<u8>) -> Option<(usize, usize)> {
    let mut seen_pages: HashSet<u8> = HashSet::with_capacity(update.len());
    let mut disallowed_pages: HashMap<u8, Vec<usize>> = HashMap::with_capacity(update.len());
    for (page_idx, page) in update.iter().enumerate() {
        if disallowed_pages.contains_key(page) {
            return Some((page_idx, disallowed_pages.get(page).unwrap()[0]));
        }
        match rules.get(page) {
            Some(preceeding_pages) => {
                for prev_page in preceeding_pages {
                    if !seen_pages.contains(prev_page) {
                        match disallowed_pages.entry(*prev_page) {
                            std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                                occupied_entry.get_mut().push(page_idx)
                            }
                            std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                                vacant_entry.insert(vec![page_idx]);
                            }
                        }
                    } else {
                    }
                }
            }
            None => (),
        }
        seen_pages.insert(*page);
    }
    None
}

// fn print_is_valid(rules: &HashMap<u8, Vec<u8>>, update: &Vec<u8>) -> bool {
//     let mut seen_pages: HashSet<u8> = HashSet::with_capacity(update.len());
//     let mut disallowed_pages: HashMap<u8, Vec<u8>> = HashMap::with_capacity(update.len());
//     for page in update {
//         if disallowed_pages.contains_key(page) {
//             let raw_input = std::fs::read_to_string("inputs/5").unwrap();
//             print!("UPDATE: [");
//             for val in update {
//                 if val == page {
//                     print!("\x1b[31m");
//                 } else if disallowed_pages.get(page).unwrap().contains(val) {
//                     print!("\x1b[32m");
//                 }
//                 print!("{}, \x1b[0m", val);
//             }
//             println!("]");
//             print!("\tbreaks rule(s):");
//             for p in disallowed_pages.get(page).unwrap() {
//                 print!(" \x1b[34m{}|{}\x1b[0m", page, p);
//                 print!(" ({})", raw_input.contains(format!("{}|{}", page, p).as_str()));
//             }
//             println!("");
//             return false
//         }
//         match rules.get(page) {
//             Some(preceeding_pages) => {
//                 for prev_page in preceeding_pages {
//                     if !seen_pages.contains(prev_page) {
//                         match disallowed_pages.entry(*prev_page) {
//                             std::collections::hash_map::Entry::Occupied(mut occupied_entry) => occupied_entry.get_mut().push(*page),
//                             std::collections::hash_map::Entry::Vacant(vacant_entry) => {vacant_entry.insert(vec![*page]);},
//                         }
//                     }
//                 }
//             },
//             None => (),
//         }
//         seen_pages.insert(*page);
//     }
//     true
// }

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn check_day05_part1_case1() {
        assert_eq!(Day05::solve_part_one(EXAMPLE_INPUT), "143".to_string())
    }

    #[test]
    fn check_day05_part2_case1() {
        assert_eq!(Day05::solve_part_two(EXAMPLE_INPUT), "123".to_string())
    }
}
