use proconio::input;

fn main() {
    input! {
        n: usize,
        wv: [(usize, usize); n],
        w: usize
    }

    let wv: Vec<(usize, usize)> = wv;
    let mut dp: Vec<Vec<usize>> = vec![vec![0; n + 1]; w + 1];

    for i in 1..=w {
        for j in 1..=n {
            let w = wv[j - 1].0;
            let v = wv[j - 1].1;

            if w <= i {
                dp[i][j] = std::cmp::max(dp[i][j - 1], v + dp[i - w][j - 1]);
            } else {
                dp[i][j] = dp[i][j - 1];
            }
        }
    }

    println!("{}", dp[w][n]);
}
