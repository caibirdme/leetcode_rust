use std::cmp::max;

impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let un = n as usize;
        if n == 1 {
            return 1;
        }
        if k == 1 {
            return n;
        }
        let mut f = vec![vec![None; k as usize+1]; un+1];
        Self::dfs(un, k as usize, &mut f)
    }
    fn dfs(n: usize, k: usize, f: &mut Vec<Vec<Option<i32>>>) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        if k == 1 {
            return n as i32;
        }
        if let Some(v) = f[n][k] {
            return v;
        }
        if k > n {
            return Self::dfs(n,n,f);
        }
        let mut ans = n as i32;
        let mut l = 1;
        let mut r = n;
        while l <= r {
            let mid = (l+r)/2;
            let broken = Self::dfs(mid-1, k-1, f);
            let well = Self::dfs(n-mid, k, f);
            if broken == well {
                ans = broken;
                break;
            } else if broken > well {
                r = mid-1;
                ans = ans.min(broken);
            } else {
                l = mid+1;
                ans = ans.min(well);
            }
        }
        ans += 1;
        f[n][k] = Some(ans);
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_super_egg_drop() {
        let test_cases = vec![
            (4, 10, 4),
            (4, 10000, 23),
            (4, 1000, 13),
            (2,4,3),
            (2,5,3),
            (1,2,2),
            (2,6,3),
            (3,14,4),
        ];
        for (k,n,expect) in test_cases {
            assert_eq!(Solution::super_egg_drop(k,n), expect, "k: {}, n: {}", k, n);
        }
    }
}