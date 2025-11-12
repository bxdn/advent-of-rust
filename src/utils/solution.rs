use std::{fs::read_to_string};

pub struct Solution {
    pub year: u16, 
    pub day: u8, 
    pub part: u8, 
    pub calculator: fn(&str) -> Result<String, SolutionErr>
}

#[derive(Debug)]
pub struct SolutionErr {pub msg: String}

impl <E> From<E> for SolutionErr 
where  E: std::error::Error {
    fn from(value: E) -> Self {
        Self{msg: value.to_string()}
    }
}

impl SolutionErr {
    pub fn new(msg: &str) -> Self {
        Self { msg: msg.to_owned() }
    }
}

impl Solution {
    pub fn calculate(&self) -> Result<String, SolutionErr> {
        let path = format!("private/input/{}/day{}.txt", self.year, self.day);
        let contents = read_to_string(path)?;
        (self.calculator)(&contents)
    }
}