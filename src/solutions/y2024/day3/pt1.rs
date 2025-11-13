
use regex::{Captures, Regex};

use crate::{utils::solution::{Solution, SolutionErr}};


pub fn sol() -> Solution {
    Solution {
        year: 2024,
        day: 3,
        part: 1,
        calculator: calculate
    }
}

fn calculate(input: &str) -> Result<String, SolutionErr> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)")?;
    Ok(re.captures_iter(input).try_fold(0, sum_regex)?.to_string())
}

fn sum_regex(total: u32, captures: Captures) -> Result<u32, SolutionErr> {
    let (_, [left, right]) = captures.extract();
    Ok(total + left.parse::<u32>()? * right.parse::<u32>()?)
}
