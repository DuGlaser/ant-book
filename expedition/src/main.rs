use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        l: i64,
        p: i64,
        a: [i64; n],
        b: [i64; n],
    }

    let mut a: Vec<i64> = a;
    a.push(l);
    let mut b: Vec<i64> = b;
    b.push(0);

    let mut p = p;
    let mut now_d = 0;
    let mut result = 0;
    let mut que = BinaryHeap::new();

    for i in 0..=n {
        let d = a[i] - now_d;

        while p - d < 0 {
            p += que.pop().unwrap();
            result += 1;
        }

        p -= d;
        que.push(b[i]);
        now_d = a[i];
    }

    println!("{}", result);
}
