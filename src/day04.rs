// use std::collections::VecDeque;

// use once_cell::sync::Lazy;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day04;

// const XMAS: Lazy<VecDeque<char>> = Lazy::new(|| VecDeque::from(['X', 'M', 'A', 'S']));
// const SAMX: Lazy<VecDeque<char>> = Lazy::new(|| VecDeque::from(['S', 'A', 'M', 'X']));

enum WordProgress {
    None,
    X,
    Xm,
    Xma,
    S,
    Sa,
    Sam,
}

struct Tracker {
    // items: VecDeque<char>,
    count: usize,
    word_progress: WordProgress,
}

impl Tracker {
    fn new() -> Self {
        Tracker {
            count: 0,
            word_progress: WordProgress::None,
        }
    }

    // fn update(&mut self, new_char: char) {
    //     self.items.push_back(new_char);
    //     if self.items.len() > 4 {
    //         self.items.pop_front();
    //     }
    //     if self.items == *XMAS || self.items == *SAMX {
    //         self.count += 1;
    //     }
    // }

    fn update(&mut self, new_char: char) {
        match new_char {
            'X' => {
                match self.word_progress {
                    WordProgress::Sam => self.count += 1,
                    _ => (),
                }
                self.word_progress = WordProgress::X;
            }
            'M' => match self.word_progress {
                WordProgress::X => self.word_progress = WordProgress::Xm,
                WordProgress::Sa => self.word_progress = WordProgress::Sam,
                _ => self.word_progress = WordProgress::None,
            },
            'A' => match self.word_progress {
                WordProgress::Xm => self.word_progress = WordProgress::Xma,
                WordProgress::S => self.word_progress = WordProgress::Sa,
                _ => self.word_progress = WordProgress::None,
            },
            'S' => {
                match self.word_progress {
                    WordProgress::Xma => self.count += 1,
                    _ => (),
                }
                self.word_progress = WordProgress::S;
            }
            _ => panic!(),
        }
    }

    fn new_line(&mut self) {
        self.word_progress = WordProgress::None;
    }
}

impl Solution for Day04 {
    type ParsedInput = Vec<Vec<char>>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        let mut input = Vec::with_capacity(input_lines.lines().count());
        for line in input_lines.lines() {
            let mut new_line = Vec::with_capacity(line.len());
            for c in line.chars() {
                new_line.push(c);
            }
            input.push(new_line);
        }
        input
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        let n_cols = parsed_input[0].len();
        let n_rows = parsed_input.len();

        // Rows
        let mut tracker = Tracker::new();
        for row_idx in 0..n_rows {
            for col_idx in 0..n_cols {
                tracker.update(parsed_input[row_idx][col_idx]);
            }
            tracker.new_line();
        }

        // Cols
        for col_idx in 0..n_cols {
            for row_idx in 0..n_rows {
                tracker.update(parsed_input[row_idx][col_idx]);
            }
            tracker.new_line();
        }

        // TL -> BR diagonals
        for row_idx in 0..(n_rows - 3) {
            let mut i = 0;
            // TODO: one of these checks is likely superfluous
            while i < n_cols && i + row_idx < n_rows {
                tracker.update(parsed_input[i + row_idx][i]);
                i += 1;
            }
            tracker.new_line();
        }
        for col_idx in 1..(n_cols - 3) {
            let mut i = 0;
            while i + col_idx < n_cols && i < n_rows {
                tracker.update(parsed_input[i][i + col_idx]);
                i += 1;
            }
            tracker.new_line();
        }

        // TR -> BL diagonals
        for row_idx in 0..(n_rows - 3) {
            let mut i = 0;
            while n_cols - 1 - i > 0 && i + row_idx < n_rows {
                tracker.update(parsed_input[i + row_idx][n_cols - 1 - i]);
                i += 1;
            }
            tracker.new_line();
        }
        for col_idx in 3..(n_cols - 1) {
            let mut i = 0;
            while 2 + col_idx - 1 - i > 0 && i < n_rows {
                tracker.update(parsed_input[i][col_idx - i]);
                i += 1;
            }
            tracker.new_line();
        }

        tracker.count.to_string()
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        let n_cols = parsed_input[0].len();
        let n_rows = parsed_input.len();
        let mut count = 0;
        for box_start_col in 0..(n_cols - 2) {
            let box_end_col = box_start_col + 2;
            for box_start_row in 0..(n_rows - 2) {
                let box_end_row = box_start_row + 2;
                if parsed_input[box_start_row + 1][box_start_col + 1] == 'A' {
                    let tl = parsed_input[box_start_row][box_start_col];
                    let tr = parsed_input[box_start_row][box_end_col];
                    let bl = parsed_input[box_end_row][box_start_col];
                    let br = parsed_input[box_end_row][box_end_col];

                    // Four orientations
                    if tl == 'M' && tr == 'M' && bl == 'S' && br == 'S'
                        || tl == 'S' && tr == 'S' && bl == 'M' && br == 'M'
                        || tl == 'M' && tr == 'S' && bl == 'M' && br == 'S'
                        || tl == 'S' && tr == 'M' && bl == 'S' && br == 'M'
                    {
                        count += 1;
                    }
                }
            }
        }
        count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day04_part1_case1() {
        assert_eq!(
            Day04::solve_part_one(
                "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            ),
            "18".to_string()
        )
    }

    #[test]
    fn check_day04_part2_case1() {
        assert_eq!(
            Day04::solve_part_two(
                "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            ),
            "9".to_string()
        )
    }
}
