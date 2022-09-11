
fn gcd(a: u32, b: u32) -> u32
{
    let mut a_ = a;
    let mut b_ = b;
    while a_ != 0 {
        let tmp = a_;
        a_ = b_ % a_;
        b_ = tmp;
    }
    return b_;
    //if a == 0{
    //    return b;
    //}
    //gcd(b % a, a)
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

    if v.len() == 2
    {
        println!("{}", v[0].max(v[1]));
        return ;
    }

    let mut prefix = Vec::with_capacity(v.len());
    let mut suffix = Vec::with_capacity(v.len());


    v.iter().fold(v[0], |acc, &val|{
        let key = gcd(acc, val);
        prefix.push(key);
        key
    });

    v.iter().rev().fold(*v.last().unwrap(), |acc, &val|{
        let key = gcd(acc, val);
        suffix.push(key);
        key
    });

    suffix.reverse();
    let suf = suffix[1];
    //println!("{prefix:?}");
    //println!("{suffix:?}");

    let mut max = prefix[prefix.len() -2].max(suf);
    for i in 1..(v.len() - 1) {
        let pre = prefix[i - 1];
        let suf = suffix[i + 1];
        let ngcd = gcd(pre, suf);
        max = ngcd.max(max);
    }
    println!("{}", max);
}
