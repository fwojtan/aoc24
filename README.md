# AdventOfCode 2024!

Template adapted by Finlay Wojtan from a [previous template](https://github.com/CastleQuirm/AdventOfCodeTemplate) by Simon Castle (which was itself adapted from a previous template by Chris Paterson).

## How to Use

### Get setup
1. Set up your Rust environment
2. Clone this template (`git clone https://github.com/fwojtan/AoC-bench-template.git`)
3. Test you can run the code, using the Day 00 example.
    - In the terminal, go to the directory you've cloned the repo into (the directory containing this README.md file)
    - Run `cargo run 0`
        - This should show some build output (the first time this is run), followed by 
        > Day 0
        >
        > Part 1: 5971
        >
        > Part 2: 1155077
        >
        > 0.024ms (exact time may vary)
        > \----------
    - Run `cargo test 00`
        - This should show some build output (the first time this is run), followed by 
        > running 3 tests
        >
        > test day00::tests::check_day00_both_case1 ... ok
        >
        > test day00::tests::check_day00_part1_case1 ... ok
        >
        > test day00::tests::check_day00_part2_case1 ... ok
        >
        > test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 75 filtered out; finished in 0.00s
4. Optionally, in order to use the benchmarking functionality, please install `valgrind` e.g. `sudo apt install valgrind`.

### Solving puzzles
Start implementing solutions!
    - Copy and paste your input for the day (e.g. [2020 Day 1's input](https://adventofcode.com/2020/day/1/input)) into the matching numbered file in the inputs directory
    - Implement the solution in the matching numbered dayXX.rs file in src
        - Run the program using `cargo run` (with the day number to run just that one day, rather than all of 1-25).  Add `--release` to perform a release build for a faster run!
    - (Optional) Add examples from the puzzle statement into tests in the same file.
        - Run the tests using `cargo test` (with the day number to run just the appropriate tests, rather than the tests for every day).

### Benchmarking
Pass `--bench` when running (e.g. `cargo run 0 --bench`) to benchmark your code using [iai](https://github.com/bheisler/iai). For the purposes of benchmarking, each solution is split into `parse_input`, `part_one` and `part_two`. 

## Notes on 2024

### Day 2
Interesting (and slightly annoying) twist for a day 2 puzzle. I ended up brute forcing it before refactoring and optimizing. Total runtime of about 220μs. Most of that is in the parsing. Going from my brute-force to smarter solution for part 2 led to ~30% speedup.

### Day 3
Is it me or is this rather fiendish for this early in the month..! My bad for taking the opportunity to try and learn `nom` I guess. Initial working part 1 was fairly slow (200+μs) because I was allocating space for data I was discarding with `many_till`. A refactor sped up a lot when manually skipping data:
```
parsing_and_part_one
  Instructions:              925202 (-59.47754%)
  L1 Accesses:              1336183 (-59.06385%)
  L2 Accesses:                   71 (-44.09449%)
  RAM Accesses:                 566 (-30.55215%)
  Estimated Cycles:         1356348 (-58.81400%)
```
Then inevitably with more complex parsing for part 2, that win is a bit less good...
```
parsing_and_part_one
  Instructions:             1399924 (+51.31009%)
  L1 Accesses:              2014202 (+50.74297%)
  L2 Accesses:                   81 (+14.08451%)
  RAM Accesses:                 590 (+4.240283%)
  Estimated Cycles:         2035257 (+50.05419%)
```
Still... pretty zippy, less than 120μs total (almost entirely on parsing).

### Day 4
Bit of a fiddly one. Initial part 1 solution was messy and slow. Used a `VecDeque` to track 4 last seen items. Part 2 solution was basically entirely independent of part 1 :(

Then went back to my part 1 and replaced my `VecDeque` with an enum-based FSM to track progress through the word. This was a *10x* speedup! Yay!