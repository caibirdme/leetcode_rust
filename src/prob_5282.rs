use std::collections::HashSet;

impl Solution {
    pub fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        if m == 0 {
            return -1;
        }
        let n = mat[0].len();
        if m == 1 && n == 1 {
            if mat[0][0] == 0 {
                return 0;
            }
            return 1;
        }
        let initial_hash = Self::hash(&mat);
        if initial_hash == 0 {
            return 0;
        }
        let mut used = HashSet::new();
        used.insert(initial_hash);
        let mut queue = vec![(initial_hash, 0)];
        let mut head = 0;
        while head < queue.len() {
            let cur = queue[head];
            head += 1;
            let next_step = cur.1+1;
            for i in 0..m {
                for j in 0..n {
                    let next = Self::reverse(cur.0, m, n, i, j);
                    if used.contains(&next) {
                        continue;
                    }
                    if next == 0 {
                        return next_step;
                    }
                    used.insert(next);
                    queue.push((next, next_step));
                }
            }
        }
        -1
    }
    fn reverse(hash: usize, m: usize, n: usize, x: usize, y: usize) -> usize {
        let mut ans = hash;
        if x+1 < m {
            ans ^= 1<<((x+1)*n+y);
        }
        if x > 0 {
            ans ^= 1<<((x-1)*n+y);
        }
        let w = x*n+y;
        if y > 0 {
            ans ^= 1<<(w-1);
        }
        if y+1<n {
            ans ^= 1<<(w+1);
        }
        ans ^ 1<<w
    }
    fn hash(mat: &Vec<Vec<i32>>) -> usize {
        let m = mat.len();
        let n = mat[0].len();
        let mut t = 0;
        for i in 0..m {
            let w = i*n;
            for j in 0..n {
                if mat[i][j] == 1 {
                    let pos = w+j;
                    t |= 1<<pos;
                }
            }
        }
        t
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_flips() {
        let test_cases = vec![
            (vec![vec![1,1,1],vec![1,0,1],vec![0,0,0]], 6),
            (vec![vec![0,0],vec![0,1]], 3),
            (vec![vec![1,0,0],vec![1,0,0]], -1),
        ];
        for (nums, expect) in test_cases {
            assert_eq!(Solution::min_flips(nums.clone()), expect, "nums: {:?}", nums);
        }
    }
}