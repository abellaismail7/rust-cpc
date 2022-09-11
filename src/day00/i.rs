use std::collections::BinaryHeap;

pub fn main() {
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line).unwrap();

    let v: Vec<usize> = line
        .split_whitespace()
        .take(2)
        .map(|s|{
            s.parse().unwrap()
        }).collect();
    let (n,k) = (v[0], v[1]);
    line = String::new();
    std::io::stdin()
        .read_line(&mut line).unwrap();
    let v: Vec<i64> = line
        .split_whitespace()
        .take(n)
        .map(|s|{
            s.parse::<i64>().unwrap()
        }).collect();

    let mut qs:BinaryHeap<i64> = BinaryHeap::new();
    for _ in 0..k{
        qs.push(0);
    }
    for item in v {
        *qs.peek_mut().unwrap() -= item;
    }
    while qs.len() > 1 {
        qs.pop();
    }
    match qs.peek() {
        Some(val) => (println!("{}", val * -1)),
        None => (println!("0")),
    }
}
