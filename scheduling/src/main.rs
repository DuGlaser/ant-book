use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
        t: [usize; n]
    };

    let s: Vec<usize> = s;
    let t: Vec<usize> = t;

    let mut s: Vec<(usize, usize)> = s
        .into_iter()
        .zip(t.into_iter())
        .collect::<Vec<(usize, usize)>>();

    s.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    let mut base = 0;
    let mut count = 0;

    for si in s.iter() {
        if si.0 > base {
            base = si.1;
            count += 1;
        }
    }

    println!("{}", count);
}
