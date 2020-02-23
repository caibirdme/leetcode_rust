impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut cnt = [0; 3];
        let n = s.len();
        let bs = s.as_bytes();
        let mut i = 0;
        let mut j = 0;
        let mut ans = 0;
        while i < n {
            while j < n && !Self::is_ok(&cnt) {
                cnt[(bs[j]-b'a') as usize] += 1;
                j+=1;
            }
            if Self::is_ok(&cnt) {
                ans += n-j+1;
            }
            cnt[(bs[i]-b'a') as usize] -= 1;
            i += 1;
        }
        ans as i32
    }
    fn is_ok(cnt: &[i32;3]) -> bool {
        cnt[1] > 0 && cnt[2] > 0 && cnt[0] > 0
    }
}

struct Solution;