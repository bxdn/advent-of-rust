use crate::{utils::solution::{Solution, SolutionErr}};


pub fn sol() -> Solution {
    Solution {
        year: 2024,
        day: 1,
        part: 1,
        calculator: pt1
    }
}

fn pt1(input: &str) -> Result<String, SolutionErr> {
    let nums = input.split_whitespace().map(str::parse::<u32>);
    let mut l = nums.clone().step_by(2).collect::<Result<Vec<_>, _>>()?;
    let mut r = nums.skip(1).step_by(2).collect::<Result<Vec<_>, _>>()?;
    l.sort_unstable();
    r.sort_unstable();
    Ok(l.into_iter()
        .zip(r)
        .map(|(l, r)| l.abs_diff(r))
        .sum::<u32>()
        .to_string()
    )
}