use proconio::input;

struct UnionFind {
    size: usize,
    pos: Vec<isize>,
}

impl UnionFind {
    /// UnionFindを作る関数
    fn new(n: usize) -> Self {
        let size = n;
        let pos = vec![-1; n + 1];
        UnionFind { size, pos }
    }

    /// rootを探す関数
    fn find(&mut self, x: usize) -> usize {
        if self.pos[x] < 0 {
            x
        } else {
            let v = self.pos[x] as usize;
            self.pos[x] = self.find(v) as isize;
            self.pos[x] as usize
        }
    }

    /// xとyの木を結合する関数
    fn unite(&mut self, x: usize, y: usize) -> Result<(), ()> {
        let mut x = self.find(x);
        let mut y = self.find(y);

        // x, y共にrootが設定済みで尚且つ
        // rootが同じ時
        if x == y {
            return Err(());
        }
        if self.pos[x] > self.pos[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.pos[x] += self.pos[y];
        self.pos[y] = x as isize;

        Ok(())
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn size(&mut self, x: usize) -> usize {
        let x = self.find(x);
        -self.pos[x] as usize
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    // 1 => A-1
    // 1 + n => B-1
    // 1 + 2n => C-1
    let mut uf = UnionFind::new(n * 3 + 1);

    let mut ans = 0;

    for _ in 0..k {
        input! {
            t: usize,
            x: usize,
            y: usize,
        }

        // 入力値にマイナスが含まれないこと前提
        if x > n || y > n {
            ans += 1;
            continue;
        }

        if t == 1 {
            if uf.same(x, y + n) || uf.same(x, y + 2 * n) {
                ans += 1;
                continue;
            }

            let _ = uf.unite(x, y);
            let _ = uf.unite(x + n, y + n);
            let _ = uf.unite(x + 2 * n, y + 2 * n);
        } else {
            if uf.same(x, y) || uf.same(x, y + 2 * n) {
                ans += 1;
                continue;
            }

            // type1の時に矛盾があるかどうかを判断するために
            // 下のような併合を行う
            let _ = uf.unite(x, y + n);
            let _ = uf.unite(x + n, y + 2 * n);
            let _ = uf.unite(x + 2 * n, y);
        }
    }

    println!("{}", ans);
}
