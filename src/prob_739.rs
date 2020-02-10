impl Solution {
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let n = t.len();
        if n == 0 {
            return vec![];
        }
        if n == 1 {
            return vec![0];
        }
        let mut ans:Vec<i32> = vec![0; n];
        let mut stack: Vec<usize> = vec![n-1];
        for i in (0..n-1).rev() {
            if t[i]<t[i+1] {
                ans[i] = 1;
            } else {
                while let Some(&v) = stack.last() {
                    if t[v] <= t[i] {
                        stack.pop();
                    } else {
                        break;
                    }
                }
                if let Some(&v) = stack.last() {
                    ans[i] = (v-i) as i32;
                }
            }
            stack.push(i);
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_daily_temperatures() {
        let test_cases = vec![
            (vec![34,80,80,34,34,80,80,80,80,34], vec![1, 0, 0, 2, 1, 0, 0, 0, 0, 0]),
            (vec![73, 74, 75, 71, 69, 72, 76, 73], vec![1, 1, 4, 2, 1, 1, 0, 0]),
        ];
        for (t, expect) in test_cases {
            assert_eq!(Solution::daily_temperatures(t.clone()), expect, "t: {:?}", t);
        }
    }
}