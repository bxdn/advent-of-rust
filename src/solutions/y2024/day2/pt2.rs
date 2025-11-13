
use crate::{solutions::y2024::day2::shared::{is_line_valid, get_nums}, utils::solution::{Solution, SolutionErr}};

pub fn sol() -> Solution {
    Solution {
        year: 2024,
        day: 2,
        part: 2,
        calculator: calculate
    }
}

fn calculate(input: &str) -> Result<String, SolutionErr> {
    Ok(input
        .lines()
        .try_fold(0, process_line)?
        .to_string())
}

fn process_line(acc: u32, line: &str) -> Result<u32, SolutionErr> {
    let nums = get_nums(line)?;
    if is_line_valid(&nums, None) {
        return Ok(acc + 1);
    }
    for i in 0..nums.len() {
        if is_line_valid(&nums, Some(i)) {
            return Ok(acc + 1);
        }
    }
    Ok(acc)
}
