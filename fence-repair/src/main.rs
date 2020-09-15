use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        l: [i32; n]
    };

    let mut h = BinaryHeap::new();

    for i in l.iter() {
        h.push(Reverse(*i));
    }

    let mut ans = 0;
    while h.len() > 1 {
        let x1: Reverse<i32> = h.pop().unwrap();
        let x2: Reverse<i32> = h.pop().unwrap();

        let sum = x1.0 + x2.0;
        ans += sum;

        h.push(Reverse(sum));
    }

    println!("{}", ans);
}

// fn main() {
//     input! {
//         n: usize,
//         l: [i32; n]
//     };

//     let mut l: Vec<i32> = l;
//     l.sort();

//     let mut sum = 0;
//     while l.len() > 1 {
//         let t = l[0] + l[1];
//         sum += t;

//         let mut ls: Vec<i32> = l[2..].to_vec();
//         ls.push(t);
//         ls.sort();

//         l = ls;
//     }

//     println!("{}", sum);
// }
