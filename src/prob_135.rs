impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        if n < 2 {
            return n as i32;
        }
        let mut v = vec![1; n];

        for i in 1..n {
            if ratings[i] > ratings[i-1] {v[i] = v[i-1]+1;}
        }
        for i in (0..n-1).rev() {
            if ratings[i] > ratings[i+1] {v[i] = v[i].max(v[i+1]+1);}
        }
        v.iter().sum()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_candy() {
        let test_cases = vec![
            (vec![1,0,2],5),
            (vec![1,2,3,4,4,3,2,1], 20),
            (vec![1,2,3,4,3,2,1], 16),
            (vec![5,4,3,2,1], 15),
            (vec![5,5,5,5,5], 5),
            (vec![5,4,4,3,3,3], 8),
            (vec![5,4,4,3,3,8], 9),
            (vec![5,4,4,3,3,2], 9),
            (vec![1,2,2],4),
        ];
        for (ratings, expect) in test_cases {
            assert_eq!(Solution::candy(ratings.clone()), expect, "ratings: {:?}", ratings);
        }
    }
}