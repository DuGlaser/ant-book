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

    /// xとyの要素を合併する
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
        txy: [(usize, usize, usize); k]
    }

    let txy: Vec<(usize, usize, usize)> = txy;

    // x, x + n, x + 2 * n をx-A, x-B, x-Cとする
    let mut uf = UnionFind::new(n * 3);

    let mut ans = 0;

    for i in txy.iter() {
        if i.1 == 0 || i.2 == 0 {
            ans += 1;
            continue;
        }

        let t = i.0;
        let x = i.1 - 1;
        let y = i.2 - 1;

        if n <= x || n <= y {
            ans += 1;
            continue;
        }

        if t == 1 {
            if uf.same(x, y + n) || uf.same(x, y + 2 * n) {
                ans += 1;
            } else {
                uf.unite(x, y);
                uf.unite(x + n, y + n);
                uf.unite(x + n * 2, y + n * 2);
            }
        } else {
            if uf.same(x, y) || uf.same(x, y + 2 * n) {
                ans += 1;
            } else {
                uf.unite(x, y + n);
                uf.unite(x + n, y + 2 * n);
                uf.unite(x + n * 2, y);
            }
        }
    }

    println!("{}", ans);
}
