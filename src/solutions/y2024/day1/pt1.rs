use crate::utils::solution::{Solution, SolutionErr};


pub fn sol() -> Solution {
    Solution {
        year: 2024,
        day: 1,
        part: 1,
        calculator: calculate
    }
}

fn calculate(input: &str) -> Result<String, SolutionErr> {
    let (mut l, mut r) = input
        .lines()
        .try_fold((Vec::new(), Vec::new()), process_line)?;

    l.sort_unstable();
    r.sort_unstable();

    Ok(l
        .iter()
        .zip(r.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum::<u32>()
        .to_string()
    )
}

fn process_line((mut l, mut r): (Vec<u32>, Vec<u32>), line: &str) -> Result<(Vec<u32>, Vec<u32>), SolutionErr> {
     let (left_str, right_str) = line
            .split_once("   ")
            .ok_or_else(|| SolutionErr::new("Malformed data: line was not in the correct shape"))?;
    l.push(left_str.parse()?);
    r.push(right_str.parse()?);
    Ok((l, r))
}