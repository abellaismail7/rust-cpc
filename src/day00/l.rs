use std::collections::VecDeque;

fn get_pair(c: u8)-> u8
{
    match c {
        b')' => b'(',
        b']' => b'[',
        _ => 0,
    }
}

fn check_balance(line: &str) -> bool
{
    let mut st: VecDeque<u8> = VecDeque::new();
    for c in line.bytes().take_while(|&u| u != 10) {
        let pair = get_pair(c);
        if pair == 0
        {
            st.push_front(c); 
            continue;
        }
        if st.is_empty() || &pair != st.front().unwrap()
        {
            return false;
        }
        st.pop_front();
    }
    st.is_empty()
}

pub fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n = line.trim().parse::<u32>().unwrap();

    for _ in 0..n  {
        line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        println!("{}", check_balance(&line));
    }
}
