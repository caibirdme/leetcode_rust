impl Solution {
    pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
        let mut cur = 0;
        let t = 2i32.pow(n as u32) as usize;
        let mut ans = vec![0];
        let mut j = 0;
        for i in 1..=t {
            if i&1==1 {
                cur ^= 1;
                ans.push(cur);
            } else {
                 cur ^= ((cur ^ (cur-1))+1);
                ans.push(cur);
            }
            if cur == start {
                j = i;
            }
        }
        let mut final_ans = vec![];
        for i in j..t {
            final_ans.push(ans[i]);
        }
        for i in 0..j {
            final_ans.push(ans[i]);
        }
        final_ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_circular_permutation() {
        let test_cases = vec![
            (2, 3, vec![3,2,0,1]),
            (3, 2, vec![2,6,7,5,4,0,1,3]),
        ];
        for (n, start, expect) in test_cases {
            assert_eq!(Solution::circular_permutation(n, start), expect, "n: {}, start: {}", n, start);
        }
    }
}