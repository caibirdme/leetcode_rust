impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut ans = Vec::with_capacity(n as usize);
        for i in 1..=9 {
            if i > n {
                break;
            }
            ans.push(i);
            Self::dfs(i, n, &mut ans);
        }
        ans
    }
    fn dfs(t: i32, n: i32, ans: &mut Vec<i32>) {
        let t = t * 10;
        for i in 0..=9 {
            let v = t+i;
            if v <= n {
                ans.push(v);
                Self::dfs(v,n,ans);
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_lexical_order() {
        let test_cases = vec![
            (1, vec![1]),
            (13, vec![1,10,11,12,13,2,3,4,5,6,7,8,9]),
            (20, vec![1,10,11,12,13,14,15,16,17,18,19,2,20,3,4,5,6,7,8,9]),
        ];
        for (n,expect) in test_cases {
            assert_eq!(expect, Solution::lexical_order(n), "n: {}", n);
        }
    }
}