use proconio::input;

fn main() {
    input! {
        n: usize,
        wv: [(usize, usize); n],
        w: usize
    }

    let mut wv: Vec<(usize, usize)> = wv;
    wv.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));

    let max = wv[0].1 * n + 1;
    let mut dp = vec![vec![0; n + 1]; max];

    for i in 1..max {
        dp[i][0] = std::u32::MAX as usize;
        for j in 1..=n {
            let w = wv[j - 1].0;
            let v = wv[j - 1].1;

            if v <= i {
                dp[i][j] = std::cmp::min(dp[i][j - 1], w + dp[i - v][j - 1]);
            } else {
                dp[i][j] = dp[i][j - 1];
            }
        }
    }

    let mut result = 0;
    for (v, dpc) in dp.iter().enumerate() {
        for &dpi in dpc.iter() {
            if dpi <= w && dpi != 0 {
                result = v;
            }
        }
    }

    println!("{}", result);
}
