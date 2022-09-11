use std::io;

pub fn main() {
    let mut set: Vec<String> = Vec::new();

    io::stdin().lines().map(|l|{
        match l {
            Ok(line) => line,
            Err(_)=> "".to_string(),
        }
    }).for_each(|l| {
        l.trim()
            .to_lowercase()
            .chars()
            .map(|c|
                if !c.is_alphabetic() {
                    return ' ';
                }else { return c;}
            )
            .collect::<String>()
            .split_whitespace()
            .for_each(|s| {
                set.push(s.to_string());
            });
    });
    set.sort();
    set.dedup();
    set.iter().for_each(|l| {
        println!("{}", l);
    });
}
