use std::collections::HashMap;

use crate::utils::solution::{Solution, SolutionErr};


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
        .try_fold((HashMap::new(), HashMap::new()), process_line)?;

    Ok(l
        .iter()
        .map(|(k, v)| *k * *v * *r.entry(*k).or_default())
        .sum::<u32>()
        .to_string()
    )
}

type Acc = (HashMap<u32, u32>, HashMap<u32, u32>);

fn process_line((mut l, mut r): Acc, line: &str) -> Result<Acc, SolutionErr> {
     let (left_str, right_str) = line
            .split_once("   ")
            .ok_or_else(|| SolutionErr::new("Malformed data: line was not in the correct shape"))?;
    *l.entry(left_str.parse()?).or_insert(0) += 1;
    *r.entry(right_str.parse()?).or_insert(0) += 1;
    Ok((l, r))
}