mod day_01_a;
mod day_01_b;

pub struct Task {
    pub id: &'static str,
    pub func: fn(input: String) -> String,
}

pub const TASKS: &[Task] = &[day_01_a::TASK, day_01_b::TASK];
