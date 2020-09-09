use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
    };

    // たて s
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            let ti = t[i - 1];
            let sj = s[j - 1];

            dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);

            if sj == ti {
                dp[i][j] += 1;
            }
        }
    }

    println!("{}", dp[m][n])
}
