use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct Elem(i32, u8);

impl Ord for Elem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Elem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut h = BinaryHeap::new();
        if a > 0 {
            h.push(Elem(a, b'a'));
        }
        if b > 0 {
            h.push(Elem(b, b'b'));
        }
        if c > 0 {
            h.push(Elem(c, b'c'));
        }
        let mut ans = vec![];
        let mut buff = None;
        while !h.is_empty() {
            let Elem(v, x) = h.pop().unwrap();
            if ans.len() < 2 {
                ans.push(x);
                if v > 1 {
                    h.push(Elem(v-1, x));
                }
                continue;
            }
            let n = ans.len();
            if ans[n-1] == ans[n-2] && ans[n-1] == x {
                buff = Some((v,x));
                continue;
            }
            ans.push(x);
            if v > 1 {
                h.push(Elem(v-1, x));
            }
            if let Some((v,x)) = buff.take() {
                h.push(Elem(v, x));
            }
        }
        unsafe {std::str::from_utf8_unchecked(&ans).to_string()}
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_longest_diverse_string() {
        let test_cases = vec![
            (7, 1, 0, "aabaa"),
            (1,1,7, "ccbccacc"),
            (2,2,1, "aabbc"),
        ];
        for (a,b,c, expect) in test_cases {
            assert_eq!(Solution::longest_diverse_string(a,b,c), expect, "a:{},b:{},c:{}",a,b,c);
        }
    }
}