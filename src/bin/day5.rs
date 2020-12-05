use aoc2020;
use aoc2020::utils::*;
use async_std::task;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_content = task::block_on(read_file_contents("day5"));
    let (duration, result) = measure(move || aoc2020::day5::part1(file_content.as_str().into()));
    print_result(result, duration);

    let file_content = task::block_on(read_file_contents("day5"));
    let (duration, result) = measure(move || aoc2020::day5::part2(file_content.as_str().into()));
    print_result(result, duration);

    Ok(())
}
