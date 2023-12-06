use super::Task;

pub const TASK: Task = Task { id: "01_a", func };

fn func(input: String) -> String {
    let total = input
        .lines()
        .map(|line| line.chars().filter_map(|char| char.to_digit(10)))
        .fold(0, |acc, numbers| {
            acc + (numbers.to_owned().next().unwrap() * 10) + numbers.last().unwrap()
        });

    return format!("{total}");
}
