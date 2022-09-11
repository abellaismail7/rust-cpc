use std::ops::Range;

fn find_sum(num: u32) -> Option<Range<u32>>
{
    let k = (num as f64).sqrt() as u32;
    for i in 2..=k {
        let tmp = (i * i + i) / 2;
        if num >= tmp && (num - tmp) % i == 0
        {
            let start = (num - tmp) / i + 1;
            return Some(start..(start + i));
        }
    }
    None
}

pub fn main(){
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n = line.trim().parse().unwrap();
    let v: Vec<u32> = std::io::stdin()
        .lines()
        .take(n)
        .map(|l| l.unwrap())
        .map(|s| s.parse().unwrap())
        .collect();
    for i in v {
        match find_sum(i) {
            Some(range) =>{
                print!("{i} = ");
                let end = range.end;
                for n in range{
                    if n == end-1
                    {
                        println!("{n}");
                        continue;
                    }
                    print!("{n} + ");
                }
            },
            None =>(println!("IMPOSSIBLE")),
        }
    }
}
