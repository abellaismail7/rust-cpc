use std::collections::HashMap;

fn get_tup(line: &str) -> (i64, i64)
{
   let vals =  line
       .split_whitespace()
       .take(2)
       .map(|s| s.parse().unwrap())
       .collect::<Vec<i64>>();
   (vals[0], vals[1])
}

pub fn main()
{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let (n, target) = get_tup(&line);
    
    line.clear();
    std::io::stdin().read_line(&mut line).unwrap();
    let v: Vec<i32> = line
        .split_whitespace()
        .take(n as usize)
        .map(|s| s.parse().unwrap())
        .collect();

    let mut sums: HashMap<i64, u32> = HashMap::with_capacity(v.len() + 1);
    sums.insert(0, 1);

    let mut total = 0_usize;
    v.iter().fold(0_i64, |acc, &val|{
        let key = acc + (val as i64);
        let k: i64 = key - target;
        if let Some(&i) = sums.get(&k) 
        {
            total += i as usize;
        }
        *sums.entry(key).or_insert(0) += 1;
        key
    });
    println!("{}", total);
}
