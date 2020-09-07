use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        s: Chars
    };

    let mut s: VecDeque<char> = s.into_iter().collect();

    let mut t: Vec<char> = Vec::new();
    for _ in 0..n {
        let s1 = s.clone().iter().collect::<String>();
        let s2 = s.clone().iter().rev().collect::<String>();

        if s1 > s2 {
            t.push(s.pop_back().unwrap());
        } else {
            t.push(s.pop_front().unwrap());
        }
    }

    println!("{}", t.iter().collect::<String>());
}
