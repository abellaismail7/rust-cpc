
fn isprime(num :u32) -> bool
{
    if num < 2{
        return false;
    }
    for i in (2..=((num as f64).sqrt() as u32)).rev()
    {
        if num % i == 0
        {
            return false;
        }
    }
    true
}

pub fn main()
{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n = line.trim().parse().unwrap();

    if isprime(n)
    {
        println!("Yes");
        return;
    }
    println!("No");
}
