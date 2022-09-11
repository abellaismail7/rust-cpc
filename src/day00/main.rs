use std::collections::VecDeque;


//#[derive(Debug)]
struct Spl<'a> {
    str: &'a str,
    index: usize,
}


impl<'a> Spl<'a> {
   fn new(str: &'a str) -> Self
   {
        Self { str, index: 0}
   }
}

impl<'a> Iterator for Spl<'a>{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.str.len() == 0
        {
            return None;
        }
        let lenght = &self.str[1..]
            .find(|c: char| c == '[' || c == ']');

        match lenght {
            Some(len) => {
                let l = len + 1;
                let tmp = Some(&self.str[..l]);
                self.str = &self.str[l..];
                tmp
            },
            None => {
                let tmp = &self.str[..];
                self.str = &self.str[(self.str.len())..];
                Some(tmp)
            },
        }
    }

}

// [ls[ab[bc]]]

// bcabls

fn y(line: &String)
{
    let mut deq:VecDeque<&str> = VecDeque::new();
    let spl = Spl::new(&line);

    for item in spl {
        let c:char = item.;
        if (c != '[') 
        {}
        else if (c != '[') 
        {

        }
        else
        {
            deq.push_back(item);
        }
    }
}

fn main()
{

    std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .for_each(|line|{
            y(&line, &mut deq);
        });

}
