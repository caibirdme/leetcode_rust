impl Solution {
    pub fn reconstruct_matrix(mut upper: i32, mut lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        let n = colsum.len();
        if n == 0 {
            return vec![];
        }
        if upper == 0 || lower == 0 {
            let mut total = upper+lower;
            let mut low = vec![0; n];
            for (i,&v) in colsum.iter().enumerate() {
                if v == 1 {
                    if total > 0 {
                        low[i] = 1;
                        total -= 1;
                    } else {
                        return vec![];
                    }
                } else if v == 2 {
                    return vec![];
                }
            }
            if upper == 0 {
                let mut ans = vec![vec![0; n]];
                ans.push(low);
                return ans;
            } else {
                let mut ans = vec![low];
                ans.push(vec![0;n]);
                return ans;
            }
        }
        let mut up = vec![0; n];
        let mut down = vec![0; n];
        for (i,&v) in colsum.iter().enumerate() {
           if v == 2 {
               up[i] = 1;
               down[i] = 1;
               if upper > 0 {
                   upper -= 1;
               } else {
                   return vec![];
               }
               if lower > 0 {
                   lower -= 1;
               } else {
                   return vec![];
               }
           }
        }
        for (i,&v) in colsum.iter().enumerate() {
           if v == 1 {
               if upper > 0 {
                   up[i] = 1;
                   upper -= 1;
               } else if lower > 0 {
                   down[i] = 1;
                   lower -= 1;
               } else {
                   return vec![];
               }
           }
        }
        if upper > 0 || lower > 0 {
            return vec![];
        }
        vec![up, down]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_reconstruct_matrix() {
        let test_cases = vec![
            (3,0,vec![1,0,2,2,1], vec![]),
            (2,1,vec![1,1,1], vec![vec![1,1,0],vec![0,0,1]]),
            (2,3,vec![2,2,1,1], vec![]),
            (5,5,vec![2,1,2,0,1,0,1,2,0,1],vec![vec![1,1,1,0,1,0,0,1,0,0],vec![1,0,1,0,0,0,1,1,0,1]]),
        ];
        for (upper,lower,colsum, expect) in test_cases {
            assert_eq!(Solution::reconstruct_matrix(upper, lower, colsum.clone()), expect, "upper: {}, lower: {}, colsum: {:?}", upper, lower, colsum);
        }
    }
}