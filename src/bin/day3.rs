use aoc2020;
use aoc2020::day3::Lines;
use aoc2020::utils::*;
use async_std::task;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data1: Lines = task::block_on(read_file("day3"))?;
    let (duration, result) = measure(move || aoc2020::day3::part1(data1));
    print_result(result, duration);

    let data2: Lines = task::block_on(read_file("day3"))?;
    let (duration, result) = measure(move || aoc2020::day3::part2(data2));
    print_result(result, duration);

    Ok(())
}
