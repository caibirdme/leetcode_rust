use std::collections::HashSet;

impl Solution {
    pub fn crack_safe(n: i32, k: i32) -> String {
        if n == 1 && k == 1 {
            return "0".to_string();
        }
        let mut ans = vec![];
        let mut hash = HashSet::new();
        let mut m = 1;
        while 2i32.pow(m) < k { m+=1;}
        let w = 2i32.pow(m)-1;
        let mut mask = 0;
        for _ in 0..n-1 {
            mask = (mask << m) | w;
        }
        Self::dfs(0, k, mask, m, &mut hash, &mut ans);
        for _ in 0..n-1 {
            ans.push(b'0');
        }
        unsafe {std::str::from_utf8_unchecked(&ans).to_string()}
    }
    fn dfs(node: i32, k: i32, mask: i32, m: u32, hash: &mut HashSet<i32>, ans: &mut Vec<u8>) {
        for i in 0..k {
            let w = (node << m) | i;
            if !hash.contains(&w) {
                hash.insert(w);
                Self::dfs(w & mask, k, mask, m, hash, ans);
                ans.push(b'0'+i as u8);
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_crack_safe() {
        let test_cases = vec![
            (1,2,"10"),
            (2,2,"01100"),
        ];
        for (n,k,expect) in test_cases {
            assert_eq!(Solution::crack_safe(n,k), expect, "n:{},k:{}",n,k);
        }
    }
}