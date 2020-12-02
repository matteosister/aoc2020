use aoc2020;
use aoc2020::day2::{PasswordDay1, PasswordDay2, Passwords};
use aoc2020::utils::*;
use async_std::task;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data1: Passwords<PasswordDay1> = task::block_on(read_file("day2"))?;
    let (duration, result) = measure(move || aoc2020::day2::first_step(data1));
    print_result(result, duration);

    let data2: Passwords<PasswordDay2> = task::block_on(read_file("day2"))?;
    let (duration, result) = measure(|| aoc2020::day2::second_step(data2));
    print_result(result, duration);

    Ok(())
}
