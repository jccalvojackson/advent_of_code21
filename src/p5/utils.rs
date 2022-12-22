use regex::Regex;
use serde::Deserialize;
use std::error::Error;

fn count_whitespace_bytes_at_start(input: &str) -> usize {
    input
        .chars()
        .take_while(|ch| ch.is_whitespace() && *ch != '\n')
        .map(|ch| ch.len_utf8())
        .sum()
}

#[derive(Debug, Deserialize)]
pub struct CrateInstruction {
    // move 5 from 4 to 5
    action: String,
    pub amount: i32,
    conn1: String,
    pub crate_orig: i32,
    conn2: String,
    pub crate_dest: i32,
}

pub fn push_to_stack(
    line: &str,
    // stacks: &mut impl Iterator<Item = Vec<i32>>,
    stacks: &mut Vec<Vec<char>>,
    idx: &mut i32,
) -> Result<(), Box<dyn Error>> {
    let nwhite = count_whitespace_bytes_at_start(line);
    *idx += (nwhite as i32) / 4;
    // let format = "[{}] {[*]}";
    let re = Regex::new(r"(\[)([A-Z])(\])(.*)").unwrap();
    let cap = re.captures_iter(line).next();
    // let (value, rest) = scan_fmt_some!(line, format, char, String);
    if let Some(unwraped_value) = cap {
        let rest: &str = &unwraped_value[4];
        let charr = &unwraped_value[2].chars().nth(0).unwrap();
        stacks[*idx as usize].push(*charr);
        if rest.is_empty() {
            Ok(())
        } else {
            push_to_stack(&rest, stacks, &mut (*idx + 1))
        }
    } else {
        Ok(())
    }
}
