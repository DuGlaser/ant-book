use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i128; n],
        k: i128
    };

    let flag = dfs(0, &a, 0, k);
    if flag {
        println!("Yes")
    } else {
        println!("No")
    }
}

fn dfs(cur_i: usize, a: &Vec<i128>, sum: i128, target: i128) -> bool {
    let mut flag = false;
    if sum == target {
        flag = true;
    } else if cur_i == a.len() {
        flag = false;
    } else {
        if dfs(cur_i + 1, a, sum + a[cur_i], target) {
            flag = true;
        };

        if dfs(cur_i + 1, a, sum, target) {
            flag = true;
        };
    }

    flag
}
