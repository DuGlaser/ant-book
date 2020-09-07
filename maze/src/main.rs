use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [Chars; n]
    };

    // 初期処理
    let mut c: Vec<Vec<char>> = c;

    for i in 0..n {
        let mut cc = c[i].clone();
        cc.push('#');
        cc.reverse();
        cc.push('#');
        cc.reverse();

        c[i] = cc;
    }

    let top_bottom_padding: Vec<char> = vec!['#'; m + 2];
    c.push(top_bottom_padding.clone());
    c.reverse();
    c.push(top_bottom_padding.clone());
    c.reverse();

    let mut start_pos = (-1, -1);

    for i in 1..=n {
        if start_pos != (-1, -1) {
            break;
        }
        for j in 1..=m {
            if c[i][j] == 'S' {
                start_pos.0 = i as i32;
                start_pos.1 = j as i32;
            }
        }
    }

    let around: Vec<(i32, i32)> = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];

    let mut q = VecDeque::new();
    q.push_back((start_pos.0 as usize, start_pos.1 as usize, 0));

    // キューを使って幅優先探索
    while q.len() != 0 {
        let q_front = q.pop_front().unwrap();
        for around_i in around.iter() {
            let y = (q_front.0 as i32 + around_i.0) as usize;
            let x = (q_front.1 as i32 + around_i.1) as usize;
            if c[y][x] == 'G' {
                println!("{}", q_front.2 + 1);
                q.clear();
                break;
            } else if c[y][x] == '.' {
                q.push_back((y, x, q_front.2 + 1));
                c[y][x] = '#';
            }
        }
    }
}
