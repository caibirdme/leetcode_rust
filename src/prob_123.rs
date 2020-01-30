use std::cmp::max;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n == 0 {
            return 0;
        }
        let mut fb: i32 = std::i32::MIN;
        let mut fs = 0;
        let mut sb = fb;
        let mut ss = 0;
        for price in prices {
            fb = fb.max(-price);
            fs = fs.max(fb+price);
            sb = sb.max(fs-price);
            ss = ss.max(sb+price);
        }
        max(0, max(fs, ss))
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_profit() {
        let test_cases = vec![
            (vec![1,3,5,4,3,7,6,9,2,4], 10),
            (vec![3,3,5,0,0,3,1,4], 6),
            (vec![1,2,4,2,5,7,2,4,9,0], 13),
            (vec![1,2,3,4,5], 4),
            (vec![7,6,4,3,1], 0),
        ];
        for (prices, expect) in test_cases {
            assert_eq!(Solution::max_profit(prices.clone()), expect, "prices: {:?}", prices);
        }
    }
}