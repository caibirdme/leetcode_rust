impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let n = coins.len();
        let max = i32::max_value();
        let mut f = vec![max; amount as usize + 1];
        f[0] = 0;
        for i in 0..n {
            let t = coins[i] as usize;
            for j in t..=amount as usize {
                if f[j-t] != max {
                    let q = f[j-t]+1;
                    if q < f[j] {
                        f[j] = q;
                    }
                }
            }
        }
        let ans = f[amount as usize];
        if ans == max {
            -1
        } else {
            ans
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_coin_change() {
        assert_eq!(
            Solution::coin_change(vec![1,2,5], 11),
            3
        );
        assert_eq!(
            Solution::coin_change(vec![2], 3),
            -1
        );
    }
}