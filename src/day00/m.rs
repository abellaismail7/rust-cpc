use std::collections::HashMap;

pub fn main()
{
    let mut set: HashMap<i32, i32> = HashMap::new();
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n = line.trim().parse::<usize>().unwrap();

    for _ in 0..n  {
        line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim()
            .split_whitespace()
            .take(n)
            .map(|s| s.parse::<i32>().unwrap())
            .for_each(|num|{
                *set.entry(num).or_insert(0) += 1;
            });
    }
    let mut del = 0;
    for (key,val) in &set
    {
        if key != val
        {
            del += val;
            if key < val
            {
                del -= key;
            }
        }
    }
    println!("{}", del);
}
