use crate::utils::solution::SolutionErr;

pub fn parse_line(line: &str) -> Result<(u32, u32), SolutionErr> {
    let num_strs: Vec<_> = line.split_whitespace().collect();
    match num_strs.as_slice() {
        [left_str, right_str] => Ok((left_str.parse()?, right_str.parse()?)),
        _ => Err(SolutionErr::new("Malformed line!"))
    }
}