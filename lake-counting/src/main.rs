use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        // 縦
        n: usize,
        // 横
        m: usize,
        l: [Chars;n]
    };

    let mut l: Vec<Vec<char>> = l;

    let top_bottom_padding = vec!['.'; m + 2];

    // (y, x)
    let around: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for i in 0..n {
        let mut lc = l[i].clone();
        lc.push('.');
        lc.reverse();
        lc.push('.');
        lc.reverse();

        l[i] = lc;
    }
    l.push(top_bottom_padding.clone());
    l.reverse();
    l.push(top_bottom_padding.clone());
    l.reverse();

    let mut count = 0;
    for i in 1..=n {
        for j in 1..=m {
            if l[i][j] == 'w' {
                count += 1;
                let mut stack: VecDeque<(usize, usize)> = VecDeque::new();
                stack.push_back((i, j));
                l[i][j] = '.';
                while stack.len() != 0 {
                    let yx = stack.pop_back().unwrap();
                    for around_i in around.iter() {
                        let y = (yx.0 as i32 + around_i.0) as usize;
                        let x = (yx.1 as i32 + around_i.1) as usize;
                        if l[y][x] == 'w' {
                            stack.push_back((y, x));
                            l[y][x] = '.';
                        }
                    }
                }
            }
        }
    }

    println!("{}", count);
}
