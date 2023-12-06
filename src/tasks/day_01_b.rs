use std::collections::HashMap;

use super::Task;

pub const TASK: Task = Task { id: "01_b", func };

const NUMBERS: &[&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn func(input: String) -> String {
    let total = input
        .lines()
        .map(|line| {
            let mut curr_str = line.to_string();
            let mut numbers: Vec<u32> = vec![];

            'outer: loop {
                let Some(first_char) = curr_str.chars().next() else {
                    break numbers;
                };

                if let Some(d) = first_char.to_digit(10) {
                    numbers.push(d);
                    curr_str.drain(0..1);
                    continue;
                }

                for (i, numstr) in NUMBERS.iter().enumerate() {
                    if curr_str.starts_with(numstr) {
                        numbers.push(i as u32);
                        curr_str.drain(0..(numstr.len()));
                        continue 'outer;
                    }
                }

                curr_str.drain(0..1);
            }
        })
        .fold(0, |acc, numbers| {
            acc + (numbers.first().expect("no first") * 10) + numbers.last().expect("no last")
        });

    return format!("{total}");
}
