use std::collections::HashMap;

pub fn main()
{
    let mut set: HashMap<String, i32> = HashMap::new();
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n = line.trim().parse::<u32>().unwrap();

    for _ in 0..n  {
        line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        if line.chars().last() == Some('\n')
        {
            line.pop();
        }
        *set.entry(line).or_insert(0) += 1;
    }
    let mut occ:i32 = 0;
    let mut name:&str = "";
    for (key,&val) in &set
    {
        if val > occ || (val == occ && key.as_str() > name)
        {
            occ = val;
            name = key;
        }
    }
    println!("{} {}", name, occ);
}
