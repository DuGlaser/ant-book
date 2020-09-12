use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        m: [i64; n],
        k: usize
    };

    let m: Vec<i64> = m;
    let mut dp: Vec<i64> = vec![-1; k + 1];

    dp[0] = 0;
    for i in 0..n {
        for j in 0..=k {
            if dp[j] >= 0 {
                dp[j] = m[i];
            } else if j >= a[i] as usize {
                if dp[j - a[i] as usize] <= 0 {
                    dp[j] = -1;
                } else {
                    dp[j] = dp[j - a[i] as usize] - 1;
                }
            }
        }
    }

    if dp[k] > 0 {
        println!("Yes")
    } else {
        println!("No")
    }
}
