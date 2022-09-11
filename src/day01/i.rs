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
    let (n, q) = get_tup(&line);
    
    line.clear();
    std::io::stdin().read_line(&mut line).unwrap();
    let v: Vec<u32> = line
        .split_whitespace()
        .take(n as usize)
        .map(|s| s.parse().unwrap())
        .collect();

    let mut sums: Vec<u64> = Vec::with_capacity(v.len() + 1);

    v.iter().fold(0_u64, |acc, &val|{
        let nacc = acc ^ (val as u64);
        sums.push(nacc);
        nacc
    });
    println!("{sums:?}");

    std::io::stdin()
        .lock()
        .lines()
        .take(q as usize)
        .map(|l| l.unwrap())
        .map( |l| get_tup(&l))
        .for_each(|(i_, j_)|{
            let i = i_ - 1;
            let j = j_ - 1;
            if i == 0
            {
                println!("{}", sums[j as usize]);
                return;
            }
            println!("{}", sums[j as usize] ^ sums[(i -1) as usize]);
        });
}
