use proconio::input;

fn main() {
    input! {
        n: usize,
        r: usize,
        x: [usize; n]
    };

    let x: Vec<usize> = x;
    let mut base_pos = x[0];
    let mut count = 0;

    loop {
        count += 1;
        let range = base_pos + r;

        let center = x.clone().into_iter().rev().find(|&a| a <= range).unwrap();

        let next_pos = x.clone().into_iter().find(|&a| a > center + r);

        if next_pos != None {
            base_pos = next_pos.unwrap();
        } else {
            break;
        }
    }

    println!("{}", count);
}
