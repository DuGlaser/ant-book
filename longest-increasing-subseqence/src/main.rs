use proconio::input;
use std::cmp::Ordering;

pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let a: Vec<usize> = a;
    let mut dp = vec![std::usize::MAX; n];

    let mut max = 0;
    for i in 0..n {
        let dpc = dp.clone();
        let index = dpc.lower_bound(&a[i]);
        dp[index] = if dp[index] <= a[i] {
            dp[index]
        } else {
            max = std::cmp::max(max, index);
            a[i]
        }
    }

    println!("{}", max + 1);
}
