
use regex::{Captures, Regex};

use crate::{utils::solution::{Solution, SolutionErr}};
pub fn sol() -> Solution {
    Solution {
        year: 2024,
        day: 3,
        part: 2,
        calculator: calculate
    }
}

fn calculate(input: &str) -> Result<String, SolutionErr> {
    let active_re = Regex::new(r"don't\(\)|mul\((\d+),(\d+)\)")?;
    let inactive_re = Regex::new(r"do\(\)")?;

    let mut cur_idx = 0;
    let mut active = true;
    let mut total = 0;

    loop {
        let re = if active {&active_re} else {&inactive_re};
        match re.captures_at(input, cur_idx) {
            None => return Ok(total.to_string()),
            Some(captures) => {
                let full_match = captures.get(0).unwrap();
                match full_match.as_str() {
                    "don't()" => active = false,
                    "do()" => active = true,
                    _ => total += get_total(&captures)?
                };
                cur_idx = full_match.end();
            }
        }
    }
}

fn get_total(captures: &Captures) -> Result<u32, SolutionErr> {
    let l_str = captures.get(1).ok_or_else(|| SolutionErr::new("Match group somehow doesn't exist!"))?.as_str();
    let r_str = captures.get(2).ok_or_else(|| SolutionErr::new("Match group somehow doesn't exist!"))?.as_str();
    Ok(l_str.parse::<u32>()? * r_str.parse::<u32>()?)
}
