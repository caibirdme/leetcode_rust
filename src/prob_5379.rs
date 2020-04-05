impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let n = stone_value.len();
        let mut f = vec![None; n];
        let mut sum = vec![0; n+1];
        for i in 0..n {
            sum[i+1] = sum[i]+stone_value[i];
        }
        let alice_can_get = Self::dp(0, &stone_value, &mut f, &sum);
        let bob = sum[n]-alice_can_get;
        if bob < alice_can_get {
            "Alice".to_string()
        } else if bob==alice_can_get {
            "Tie".to_string()
        } else{
            "Bob".to_string()
        }
    }
    fn dp(i: usize, value: &Vec<i32>, f: &mut Vec<Option<i32>>, sum: &Vec<i32>) -> i32 {
        let n = value.len();
        if i == n {
            return 0;
        }
        if i == n-1 {
            return value[n-1];
        }
        if let Some(v) = f[i] {
            return v;
        }
        let mut ans = value[i]+sum[n]-sum[i+1]-Self::dp(i+1, value, f, sum);
        let mut t = value[i];
        for j in i+1..i+3 {
            if j >= n {
                break;
            }
            t += value[j];
            let w = t+sum[n]-sum[j+1]-Self::dp(j+1, value, f, sum);
            ans = ans.max(w);
        }
        f[i] = Some(ans);
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_stone_game_iii() {
        let test_cases = vec![
            (vec![1,2,3,-1,-2,-3,7], "Alice"),
            (vec![1,2], "Alice"),
            (vec![-1,1], "Tie"),
            (vec![1,2,3,7], "Bob"),
            (vec![-1,-2,-3], "Tie"),
            (vec![1,2,3,-9], "Alice"),
            (vec![1,2,3,6], "Tie"),
        ];
        for (v, expect) in test_cases {
            assert_eq!(Solution::stone_game_iii(v.clone()), expect, "v: {:?}", v);
        }
    }
}