# Advent of Code 2023 solutions
To try it out, clone this repository with git, click on the day to go to the folder with the solution, open up a terminal and run the commands given.
For rust projects, you must have cargo installed. For fasm projects, you must have the Flat assembler installed.

### [Day 1](./day1)
Written with the Fasm assembler in x86_64 assembly in windows subsystem for linux. This will only work on x86_64 aka AMD64 processors .Should run fine in Linux.
Uses my handwritten library, `print_integer64.inc` for printing numbers. Uses no C or system libraries, just kernel calls.

#### Part 1:
- Compile part1 in WSL with `fasm part1.asm`
- Run in linux or WSL `./part1 input-file.txt`, where `input-file.txt` is the problem input

#### Part 2:
- Compile part2 in WSL with `fasm main.asm`
- Run in linux or WSL  `./main input-file.txt`, where `input-file.txt` is the problem input

### [Day 2 part 1](./day2_part1)
Written in rust. 
Run with `cargo run input-file.txt`, where `input-file.txt` is the problem input.

### [Day 2 part 2](./day2_part2)
Written in rust.
Run with `cargo run input-file.txt`, where `input-file.txt` is the problem input.

### [Day 3 part 1](./day3_part1)
Written in rust
Run with `cargo run input-file.txt` where `input-file.txt` is the problem input.