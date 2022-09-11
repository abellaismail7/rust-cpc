use std::collections::BTreeMap;


pub fn main()
{
    let mut tree = BTreeMap::new();
    std::io::stdin().lines().for_each(|l|{
        let rl = l.unwrap();
        let ins:Vec<&str> = rl
            .trim()
            .split_whitespace()
            .take(2)
            .collect::<Vec<&str>>();
        if ins.len() != 2
        {
            return ();
        }
        let index = ins[1].parse::<i32>().unwrap();
        match ins[0] {
            "insert" => {tree.insert(index, true);},
            "exists" => {println!("{}", tree.get(&index).is_some());},
            "delete" => {tree.remove(&index);},
            _ => {},
        }
        ()
    });
}

