use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Elem(i32, usize, usize);

impl Ord for Elem {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 == other.0 {
            if self.1 == other.1 {
                other.2.cmp(&self.2)
            } else {
                other.1.cmp(&self.1)
            }
        } else {
            other.0.cmp(&self.0)
        }
    }
}

impl PartialOrd for Elem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn assign_bikes(workers: Vec<Vec<i32>>, bikes: Vec<Vec<i32>>) -> Vec<i32> {
        let n = workers.len();
        if n == 0 {
            return vec![];
        }
        let m = bikes.len();
        if m == 0 {
            return vec![];
        }
        let mut hp = BinaryHeap::new();
        for i in 0..n {
            for j in 0..m {
                let d = Self::abs(workers[i][0]-bikes[j][0]) + Self::abs(workers[i][1]-bikes[j][1]);
                hp.push(Elem(d, i, j));
            }
        }
        let mut ans = vec![0; n];
        let mut used_w = vec![false; n];
        let mut used_b = vec![false; m];
        let mut count = 0;
        while let Some(Elem(d, i, j)) = hp.pop() {
            if used_w[i] || used_b[j] {continue;}
            ans[i] = j as i32;
            used_w[i] = true;
            used_b[j] = true;
            count += 1;
            if count == n {
                break;
            }
        }
        ans
    }
    #[inline]
    fn abs(x: i32) -> i32 {
        if x >= 0 {x} else {-x}
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_assign_bikes() {
        let test_cases = vec![
            (
                vec![[664,994],[3,425],[599,913],[220,352],[145,348],[604,428],[519,183],[732,148]],
                vec![[611,698],[113,338],[579,770],[276,588],[948,679],[731,525],[925,877],[182,281],[39,299]],
                vec![0,8,2,7,1,5,3,4],
            ),
            (vec![[0,0],[2,1]], vec![[1,2],[3,3]], vec![1,0]),
            (vec![[0,0],[1,1],[2,0]], vec![[1,0],[2,2],[2,1]], vec![0,2,1]),
        ];
        for (workers, bikes, expect) in test_cases {
            assert_eq!(Solution::assign_bikes(
                workers.iter().map(|v| vec![v[0], v[1]]).collect(),
                bikes.iter().map(|v| vec![v[0], v[1]]).collect()
            ), expect, "workers: {:?}, bikes: {:?}", workers, bikes);
        }
    }
}