use crate::utils::solution::SolutionErr;

struct AttemptState{all_increasing: bool, all_decreasing: bool, prev: Option<i32>}

pub fn attempt(nums: &Vec<i32>, skip: Option<usize>) -> bool {
    let mut state = AttemptState{all_increasing: true, all_decreasing: true, prev: None};
    for i in 0..nums.len() {
        state = match skip {
            Some(val) if i == val => state,
            _ => aggregate(state, nums[i])
        };
        if !state.all_increasing && !state.all_decreasing {
            return false;
        }
    }
    true
}

fn aggregate(acc: AttemptState, num: i32) -> AttemptState {
    let AttemptState{all_increasing, all_decreasing, prev} = acc;
    let Some(last) = prev else {
        return AttemptState{prev: Some(num), ..acc};
    };
    let prev = Some(num);
    match num - last {
        1..=3 => AttemptState{all_decreasing: false, all_increasing, prev},
        -3..=-1 => AttemptState{all_increasing: false, all_decreasing, prev},
        _ => AttemptState{all_decreasing: false, all_increasing: false, prev}
    }
}

pub fn get_nums(line: &str) -> Result<Vec<i32>, SolutionErr> {
    let mut res = Vec::new();
    for str in line.split_whitespace() {
        res.push(str.parse()?);
    }
    Ok(res)
}