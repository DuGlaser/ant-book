use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [i32; n]
    };

    let mut l: Vec<i32> = l;
    l.sort();

    let mut sum = 0;
    while l.len() > 1 {
        let t = l[0] + l[1];
        sum += t;

        let mut ls: Vec<i32> = l[2..].to_vec();
        ls.push(t);
        ls.sort();

        l = ls;
    }

    println!("{}", sum);
}
