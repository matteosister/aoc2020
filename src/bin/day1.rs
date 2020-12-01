use aoc2020;
use aoc2020::utils::*;
use async_std::task;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data1 = task::block_on(read_file("day1"))?;
    aoc2020::day1::one(data1);
    let data2 = task::block_on(read_file("day1"))?;
    aoc2020::day1::two(data2);

    Ok(())
}
