use proconio::input;

fn main() {
    input! {
        c: [usize; 6],
        a: usize
    };

    let coins = vec![1, 5, 10, 50, 100, 500];

    let mut count = 0;
    let mut sum = 0;

    for (ci, ci_count) in coins.iter().rev().zip(c.iter().rev()) {
        let mut remnants = *ci_count;

        if sum == a {
            break;
        }

        while remnants > 0 && a - sum >= *ci {
            sum += ci;
            count += 1;
            remnants -= 1;
        }
    }

    println!("{}", count);
}
