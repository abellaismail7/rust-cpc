use std::{error::Error, fs, cmp};

fn get_vals(file: &str) -> Result<(usize, usize), Box<dyn Error>>
{
    let line = fs::read_to_string(file)?;
    let vals:Vec<usize> = line
        .trim()
        .split_whitespace()
        .take(2)
        .map(|s| s.parse().unwrap())
        .collect();
    return Ok((vals[0], vals[1]));
}


pub fn main() -> Result<(), Box<dyn Error>>{
    let (mut a,mut b) = get_vals("input.txt")?;
    let mut str = String::new();
    let common:usize= cmp::min(a, b);

    if a > b
    {
        str.push_str(&"BG".repeat(common));
    }
    else
    {
        str.push_str(&"GB".repeat(common));
    }

    a -= common;
    b -= common;

    str.push_str(&"B".repeat(a));
    str.push_str(&"G".repeat(b));
    fs::write("output.txt", str)?;
    Ok(())
}

