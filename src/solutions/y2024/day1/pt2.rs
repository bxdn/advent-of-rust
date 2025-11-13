use std::collections::HashMap;

use crate::{solutions::y2024::day1::shared::parse_line, utils::solution::{Solution, SolutionErr}};


pub fn sol() -> Solution {
    Solution {
        year: 2024,
        day: 1,
        part: 2,
        calculator: calculate
    }
}

fn calculate(input: &str) -> Result<String, SolutionErr> {
    let (l, mut r) = input
        .lines()
        .try_fold((HashMap::new(), HashMap::new()), folder)?;

    Ok(l
        .iter()
        .map(|(k, v)| *k * *v * *r.entry(*k).or_default())
        .sum::<u32>()
        .to_string()
    )
}

type Acc = (HashMap<u32, u32>, HashMap<u32, u32>);

fn folder((mut l, mut r): Acc, line: &str) -> Result<Acc, SolutionErr> {
     let (left, right) = parse_line(line)?;
    *l.entry(left).or_insert(0) += 1;
    *r.entry(right).or_insert(0) += 1;
    Ok((l, r))
}