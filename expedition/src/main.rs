use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        l: usize,
        p: usize,
        a: [usize; n],
        b: [usize; n]
    }

    let a: Vec<usize> = a;
    let b: Vec<usize> = b;

    let mut p = p;
    let mut now_l = 0;

    let ab: Vec<(usize, usize)> = a.into_iter().zip(b.into_iter()).collect();

    let mut count = 0;

    let mut h = BinaryHeap::new();

    loop {
        now_l += p;
        if now_l >= l {
            break;
        }
        for i in ab.iter().filter(|&x| now_l - p < x.0 && x.0 <= now_l) {
            h.push(i.1);
        }
        p = h.pop().unwrap();
        count += 1;
    }

    println!("{}", count);
}
