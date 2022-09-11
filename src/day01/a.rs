use std::io::BufRead;

fn get_tup(line: &str) -> (u32, u32)
{
   let vals =  line
       .split_whitespace()
       .take(2)
       .map(|s| s.parse().unwrap())
       .collect::<Vec<u32>>();
   (vals[0], vals[1])
}

pub fn main()
{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n = line.trim().parse().unwrap();
    
    line.clear();
    std::io::stdin().read_line(&mut line).unwrap();
    let v: Vec<u32> = line
        .split_whitespace()
        .take(n)
        .map(|s| s.parse().unwrap())
        .collect();

    let mut sums: Vec<u64> = Vec::with_capacity(v.len() + 1);

    v.iter().fold(0_u64, |acc, &val|{
        sums.push(acc + (val as u64));
        acc + (val as u64)
    });

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n: u32 = line.trim().parse().unwrap();

    std::io::stdin()
        .lock()
        .lines()
        .take(n as usize)
        .map(|l| l.unwrap())
        .map( |l| get_tup(&l))
        .for_each(|(i, j)|{
            if i == 0
            {
                println!("{}", sums[j as usize]);
                return;
            }
            println!("{}", sums[j as usize] - sums[(i -1) as usize]);
        });
}
