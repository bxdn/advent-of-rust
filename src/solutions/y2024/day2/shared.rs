use crate::utils::solution::SolutionErr;

pub fn is_line_valid(nums: &Vec<i32>, skip: Option<usize>) -> bool {
    let (mut increasing, mut decreasing) = (true, true);
    let mut prev = None;
    for (i, num) in nums.iter().enumerate() {
        if let Some(val) = skip && i == val {
            continue;
        }
        (increasing, decreasing) = process_state(increasing, decreasing, prev, *num);
        if !increasing && !decreasing {
            return false;
        }
        prev = Some(*num);
    }
    true
}

fn process_state(increasing: bool, decreasing: bool, prev: Option<i32>, num: i32) -> (bool, bool) {
    let Some(last) = prev else {
        return (increasing, decreasing);
    };
    match num - last {
        1..=3 => (increasing, false),
        -3..=-1 => (false, decreasing),
        _ => (false, false)
    }
}

pub fn get_nums(line: &str) -> Result<Vec<i32>, SolutionErr> {
    let mut res = Vec::new();
    for str in line.split_whitespace() {
        res.push(str.parse()?);
    }
    Ok(res)
}