
fn factor(num: u32) -> Vec<u32>
{
    let mut n = num;
    let mut v = Vec::new();
    let sq = (n as f64).sqrt() as u32;
    for i in 2..=sq {
        while n % i == 0 {
            v.push(i);
            n /= i;  
        }
    }
    if n > 1
    {
        v.push(n);
    }
    v
}

pub fn main()
{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n = line.trim().parse().unwrap();

    let v = factor(n);
    if v.len() == 0
    {
        return ;
    }

    let mut value = *v.first().unwrap(); 
    let mut occ = 1;
    for &i in v.iter().skip(1) {
        if value != i {
            print!("{}", value);
            if occ > 1
            {
                print!("^{}", occ);
            }
            print!("*");
            value = i;
            occ = 1;
            continue;
        }
        occ += 1;

    }
    print!("{}", value);
    if occ > 1
    {
        print!("^{}", occ);
    }
    println!("");
    
}
