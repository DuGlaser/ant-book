use proconio::input;

fn main() {
    input! {
        n: usize,
        m: i32,
        l: [(usize, usize); m]
    }

    let l: Vec<(usize, usize)> = l;
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut color: Vec<i32> = vec![0; n];

    for &(i, j) in l.iter() {
        graph[i].push(j);
    }

    let mut flag = true;
    for i in 0..n {
        if color[i] == 0 {
            if !dfs(&graph, &mut color, 0, 1) {
                println!("No");
                flag = false;
            }
        }
    }

    if flag {
        println!("Yes");
    }
}

fn dfs(graph: &Vec<Vec<usize>>, color: &mut Vec<i32>, i: usize, c: i32) -> bool {
    color[i] = c;
    for &gi in graph[i].iter() {
        if color[gi] == c {
            return false;
        }

        if color[gi] == 0 && !dfs(graph, color, gi, c * -1) {
            return false;
        }
    }

    return true;
}
