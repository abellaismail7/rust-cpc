use std::collections::HashMap;

fn fun(n: u64) -> i64
{
    if n <= 2
    {
        return 1;
    }
    else if n % 2 == 1
    {
        return fun(6 * n / 7) + fun(2 * n /3);
    }
    fun(n -1) + fun(n - 3)
}

pub fn main()
{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n = line.trim().parse::<u64>().unwrap();
    println!("{}", fun(n));

}
