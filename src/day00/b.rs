fn get_arr() -> Vec<i32>
{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n = line.trim().parse().unwrap();
    line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line
        .trim()
        .split_whitespace()
        .take(n)
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn main()
{
    for i in get_arr()
    {
        if i >= 0 {
            print!("{} ", i + 2);
        }
        else
        {
            print!("{} ", i);
        }
    }
    println!("");
}
