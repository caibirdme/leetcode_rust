impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let mut arr = vec![];
        let mut uk = k as usize;
        let mut ans = None;
        Self::dfs(0, n as usize, &mut uk, &mut arr, &mut ans);
        ans.unwrap_or("".to_string())
    }
    fn dfs(i: usize, n: usize, k: &mut usize, arr: &mut Vec<u8>, ans: &mut Option<String>) {
        if ans.is_some() {
            return;
        }
        if i == n {
            *k -= 1;
            if *k == 0 {
                *ans = Some(unsafe {std::str::from_utf8_unchecked(&arr).to_string()});
            }
            return;
        }
        for c in b'a'..=b'c' {
            if i > 0 && arr[i-1] == c {
                continue;
            }
            arr.push(c);
            Self::dfs(i+1, n, k, arr, ans);
            arr.pop();
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_get_happy_string() {
        let test_cases = vec![
            (1, 4, ""),
            (3, 9, "cab"),
            (2, 7, ""),
            (10, 100, "abacbabacb"),
        ];
        for (n,k,expect) in test_cases {
            assert_eq!(Solution::get_happy_string(n, k), expect, "n:{}, k:{}",n,k);
        }
    }
}