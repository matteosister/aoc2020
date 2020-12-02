use aoc2020;
use aoc2020::day2::{PasswordDay1, Passwords};
use aoc2020::utils::*;
use async_std::task;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data2: Passwords<PasswordDay1> = task::block_on(read_file("day2"))?;
    aoc2020::day2::first_step(data2);

    Ok(())
}
