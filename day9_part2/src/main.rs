use advent_of_code_lib_rs::advent_of_code;
use advent_of_code_lib_rs::file_parser::ScannerError;
use std::iter;

fn main() {
    let mut parser = advent_of_code::scan_file(std::env::args()).unwrap();

    let line_vec_iter = iter::from_fn(move || {
        let line = parser.parse_line_to_vec_i64(' ');
        let line = match line {
            Err(ScannerError::EndOfFile) => {
                return None;
            }
            Err(e) => {
                panic!("{:?}", e);
            }
            Ok(v) => v,
        };

        return Some(line);
    });

    let all_history_values_sum = get_all_differences(line_vec_iter);

    println!(
        "Sum of all all history values for file: {}",
        all_history_values_sum
    );
}

fn get_all_differences(lines: impl Iterator<Item = Vec<i64>>) -> i64 {
    let mut tally: i64 = 0;

    for line in lines {
        let pyramid = get_line_differences2(&line);

        // work out the value at the top of another column
        let top_right = extrapolate(&pyramid);
        tally += top_right;
    }

    return tally;
}

/// Work out what another column's (on the left) top digit would be
///  I have no idea how I got this to work... I'm so confused.
fn extrapolate(pyramid: &[Vec<i64>]) -> i64 {
    let mut bellow;
    let mut tmp = 0;
    for row in pyramid.iter().rev().skip(1) {
        bellow = *row.first().unwrap();

        tmp =  - tmp + bellow;
    }

    return tmp;
}
fn get_line_differences2(line: &[i64]) -> Vec<Vec<i64>> {
    let mut all = Vec::new();
    all.push(line.to_owned());

    let mut current_line = line.to_owned();

    loop {
        let new_line = get_line_differences(&current_line);

        all.push(new_line.to_owned());

        let is_zeros = is_all_zeros(new_line.clone().into_iter());
        if !is_zeros {
            current_line = new_line.to_owned();
            continue;
        }

        break;
    }

    return all;
}

fn get_line_differences(line: &[i64]) -> Vec<i64> {
    //may cause slightly larger memory usage in trade for requiring only 1 allocation
    let mut differences = Vec::with_capacity(line.len());
    let mut line = line.iter().peekable();
    let mut peek;
    let mut num;
    loop {
        num = line.next();
        peek = line.peek();

        match (num, peek) {
            (Some(current), Some(next)) => {
                let current = *current;
                let next = **next;

                let difference = next - current;
                differences.push(difference);
            }
            _ => break,
        }
    }

    return differences;
}

fn is_all_zeros(mut numbers: impl Iterator<Item = i64>) -> bool {
    numbers.all(|n| n == 0)
}
