impl Solution {
    pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
        if s.is_empty() || words.is_empty() {
            return 0;
        }
        let count_s = Self::split(s.as_bytes());
        let mut ans = 0;
        for word in words {
            let cur = Self::split(word.as_bytes());
            if cur.len() != count_s.len() {
                continue;
            }
            let mut ok = true;
            for (&(a,b), &(c,d)) in count_s.iter().zip(cur.iter()) {
                if a != c {
                    ok = false;
                    break;
                }
                if b == d {
                    continue;
                }
                if d > b || b == 2{
                    ok = false;
                    break;
                }
            }
            if ok {
                ans += 1;
            }
        }
        ans
    }
    fn split(bs: &[u8]) -> Vec<(u8, usize)> {
        let mut cs = vec![];
        let mut i = 0;
        while i < bs.len() {
            let mut j = i;
            while j < bs.len() && bs[j] == bs[i] {j+=1;}
            cs.push((bs[i], j-i));
            i = j;
        }
        cs
    }
}

struct Solution;