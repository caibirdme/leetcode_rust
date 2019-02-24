impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        use std::cmp::{max,min};
        let n = prices.len();
        if n < 2 {
            return 0;
        }
        if n == 2 {
            let p = prices[1] - prices[0];
            if p > 0 {
                return p;
            }
            return 0;
        }
        let mut buy = vec![0; n];
        let mut sell = vec![0; n];
        sell[1] = prices[1]-prices[0];
        buy[0] = -prices[0];
        buy[1] = max(-prices[0], -prices[1]);
        for i in 2..n {
            sell[i] = max(sell[i-1], buy[i-1]+prices[i]);
            buy[i] = max(buy[i-1], sell[i-2]-prices[i]);
            buy[i] = max(buy[i], -prices[i]);
        }
        max(sell[n-1],0)
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_profit() {
        assert_eq!(
            Solution::max_profit(vec![4,2,7,1,11]),
            10
        );
        assert_eq!(
            Solution::max_profit(vec![1,2,4,1,2]),
            3
        );
        assert_eq!(
            Solution::max_profit(vec![1,2,3,0,2]),
            3
        );
        assert_eq!(
            Solution::max_profit(vec![1,2,8,10,15]),
            14
        );
        assert_eq!(
            Solution::max_profit(vec![6,5,4,3,2,1]),
            0
        );

    }
}