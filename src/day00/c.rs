
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
    let v1 = get_arr();
    let v2 = get_arr();
    let mut res:Vec<i32> = Vec::new();

    for i in v1 {
        if !v2.contains(&i)
        {
            res.push(i);
        }
    }
    println!("{}", res.len());
    for i in res{
        print!("{} ", i);
    }
    println!("");
}
