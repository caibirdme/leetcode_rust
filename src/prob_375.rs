impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        if n < 2 {
            return 0;
        }
        let n = n as usize;
        let mut f = vec![vec![0; n+1]; n+1];
        Self::dfs(1,n,&mut f)
    }
    fn dfs(l: usize, r: usize, f: &mut Vec<Vec<i32>>) -> i32 {
        use std::cmp::max;
        if l == r {
            return 0;
        }
        if l+1 == r {
            return l as i32;
        }
        if f[l][r] != 0 {
            return f[l][r];
        }
        let mut ans = std::i32::MAX;
        for i in l+1..r {
            let t = i as i32 + max(Self::dfs(l,i-1, f), Self::dfs(i+1,r, f));
            if t < ans {
                ans = t;
            }
        }
        f[l][r] = ans;
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_get_money_amount() {
        let test_cases = vec![
            (2,1),
            (3,2),
            (5,6),
            (10, 16),
        ];
        for (n,expect) in test_cases {
            assert_eq!(expect, Solution::get_money_amount(n), "n: {}", n);
        }
    }
}