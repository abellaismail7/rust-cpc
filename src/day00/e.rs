use std::collections::{BinaryHeap, HashMap};

fn pop<T>(map: &mut HashMap<i32, i32>,heap: &mut BinaryHeap<i32>, get_num: T) where T: Fn(i32)-> i32
{
    while heap.peek().is_some() {
        let &peek = heap.peek().unwrap();
        let num = get_num(peek);
        if map.get(&num).is_none()
        {
            heap.pop().unwrap();
        }
        else{break;}
    }
    match heap.pop() {
        Some(num) =>{
            let n = get_num(num);
            println!("{}", map.get(&n).unwrap());
            map.remove(&n);
        },
        None => println!("0"),
    };

}

pub fn main()
{
    let mut heap = BinaryHeap::new();
    let mut rheap = BinaryHeap::new();
    let mut map: HashMap<i32, i32> = HashMap::new();
    std::io::stdin().lines()
        .map_while(|l| {
            let res:Vec<i32> = l.unwrap().trim()
                .split_whitespace()
                .take(3)
                .map(|s| s.parse().unwrap())
                .collect();
            if res.len() == 0 || res[0] == 0
            {
                return None;
            }
            Some(res)
        })
        .for_each(|vec|{
            match vec.get(0) {
                Some(1) => {
                    heap.push(vec[2]);
                    rheap.push(vec[2] * -1);
                    map.insert(vec[2], vec[1]);
                },
                Some(2) => {
                    pop(&mut map, &mut heap, |x| x);
                },
                Some(3) => {
                    pop(&mut map, &mut rheap, |x| x * -1);
                },
                _ => {},
            }
            ()
    });
}
