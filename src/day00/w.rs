fn get_tup(line: &str) -> (i16, i16)
{
   let vals =  line
       .split_whitespace()
       .take(2)
       .map(|s| s.parse().unwrap())
       .collect::<Vec<i16>>();
   (vals[0], vals[1])
}


pub fn main()
{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)
        .unwrap();

    let (n, k) = get_tup(&line);

    let ps: Vec<(i16, i16)> = std::io::stdin()
        .lines()
        .take(n as usize)
        .map(|l| l.unwrap())
        .map( |l| get_tup(&l))
        .collect();


    let psclone = ps.iter().take(1).into_iter()
        .chain(ps.iter());
    let s = 0.0;

    let p = ps.iter().zip(psclone)
        .fold(s, |acc, ((x1, y1), (x2, y2))|{
            acc + (((x2 - x1).pow(2) + (y2 - y1).pow(2)) as f64).sqrt()
        });
    println!("{:.9}", p /50.0 * (k as f64));
}
