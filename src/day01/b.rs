
fn gcd(a: u32, b: u32) -> u32
{
    let mut a_ = a;
    let mut b_ = b;
    while a_ != 0 {
        let tmp = a_;
        a_ = b_ % a_;
        b_ = tmp;
    }
    return b_;
    //if a == 0{
    //    return b;
    //}
    //gcd(b % a, a)
}


pub fn main()
{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n = line.trim().parse().unwrap();
    
    line.clear();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut v: Vec<u32> = line
        .split_whitespace()
        .take(n)
        .map(|s| s.parse().unwrap())
        .collect();
    v.sort();

    if v.first() == Some(&1) || v.first() == Some(&2)
    {
        println!("{}", v.first().unwrap());
        return;
    }

    let n = v.iter().fold(*v.first().unwrap(), |acc, &val|{
        gcd(acc, val)
    });

    println!("{n}");
}
