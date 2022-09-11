
fn get_tup(line: &str) -> (i64, i64)
{
   let vals =  line
       .split_whitespace()
       .take(2)
       .map(|s| s.parse().unwrap())
       .collect::<Vec<i64>>();
   (vals[0], vals[1])
}


fn gcd(a: i64, b: i64) -> i64
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
    let (a,b) = get_tup(&line);
    if a == 0 && b == 0
    {
        println!("{}", 1);
        return;
    }

    println!("{}", (a * b).abs() / gcd(a, b));
}
