use std::cmp::{max, Ordering};
use std::collections::BinaryHeap;

macro_rules! idx {
    ($c: ident) => {
        (($c-b'a') as usize)
    };
}

#[derive(Eq)]
struct elem(i32,u8);

impl elem {
    fn new(total: i32, c: u8) -> Self {
        Self(total, c)
    }
}

impl Ord for elem {
    fn cmp(&self, other: &Self) -> Ordering {
        let r = self.0.cmp(&other.0);
        match r {
            Ordering::Equal => self.1.cmp(&other.1),
            _ => r,
        }
    }
}

impl PartialOrd for elem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for elem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Solution {
    pub fn rearrange_string(s: String, k: i32) -> String {
        if k == 0 {
            return s;
        }
        let uk = k as usize;
        let mut count = vec![0; 26];
        let mut largest = 0;
        for &c in s.as_bytes() {
            let pos = idx!(c);
            count[pos] += 1;
            largest = max(largest, count[pos]);
        }
        if ((largest-1)*k+1) > s.len() as i32 {
            return "".to_string();
        }
        let mut pq = BinaryHeap::new();
        for i in 0..26 {
            if count[i] > 0 {
                pq.push(elem::new(count[i], i as u8));
            }
        }
        let mut i = 0usize;
        let mut ans = vec![0u8; s.len()];
        let mut temp = vec![];
        while i < s.len() {
            temp.clear();
            for j in 0..uk {
                let pos = i+j;
                if pos >= s.len() {
                    break;
                }
                if let Some(v) = pq.pop() {
                    ans[pos] = v.1+b'a';
                    temp.push(v);
                } else {
                    return "".to_string();
                }
            }
            temp.iter().for_each(|v| {
                if v.0 > 1 {
                    pq.push(elem::new(v.0-1, v.1));
                }
            });
            i += uk;
        }
        unsafe {std::str::from_utf8_unchecked(&ans).to_string()}
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    fn check(s: &String, k: i32) -> bool {
        if s.is_empty() {
            return false;
        }
        let chs = s.as_bytes();
        let uk = k as usize;
        for i in 0u8..26 {
            let mut pre = None;
            let c = i+b'a';
            for j in 0..chs.len() {
                if chs[j] == c {
                    match pre {
                        None => pre = Some(j),
                        Some(p) => {
                            if j-p<uk {
                                return false;
                            }
                            pre = Some(j);
                        }
                    }
                }
            }
        }
        true
    }

    #[test]
    fn test_rearrange_string() {
        let test_cases = vec![
            ("aabbccdd", 4, false),
            ("bbabcaccaaabababbaaaaccbbcbacbacacccbbcaabcbcacaaccbabbbbbcacccaccbabaccbacabcabcacb", 2, false),
            ("a", 1, false),
            ("aaabc",2, false),
            ("aaabbcc", 3, false),
            ("aaabbc", 3, true),
            ("asdasdhaksdhajshdiqhwdjasdiquwdasdjgasjhdajsdjasdajs", 4, false),
            ("aaadbbcc", 2, false),
            ("aabbcc", 3, false),
            ("aaabc", 3, true),
        ];
        for (s, k, should_empty) in test_cases {
            let actual = Solution::rearrange_string(s.to_string(), k);
            if should_empty {
                assert_eq!(actual, "", "s: {}, k: {}", s,k);
            } else {
                assert!(check(&actual, k), "s: {}, k: {}", s,k);
            }
        }
    }
}