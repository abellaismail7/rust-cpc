fn get_tup(line: &str) -> (u32, u32)
{
   let vals =  line
       .split_whitespace()
       .take(2)
       .map(|s| s.parse().unwrap())
       .collect::<Vec<u32>>();
   (vals[0], vals[1])
}


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
    let (a,b) = get_tup(&line);
    let mut isabsent = true;

    for i in a..=b
    {
        if isprime(i)
        {
            isabsent = false;
            println!("{i}");
        }
    }
    if isabsent {
        println!("Absent");
    }
}
