use std::cmp::min;
macro_rules! idx {
    ($i: expr) => {
        (match $i as char {
           'Q' => 0,
           'W' => 1,
           'E' => 2,
           'R' => 3,
           _ => unreachable!(),
        }) as usize
    };
}

impl Solution {
    pub fn balanced_string(s: String) -> i32 {
        let n = s.len();
        let mut c = [0; 4];
        s.as_bytes().iter().for_each(|&ch| {
            let i = idx!(ch);
            c[i] += 1;
        });
        let mut target = [None; 4];
        let mut sum = 0;
        let piv = (n / 4) as i32;
        for i in 0..4 {
            if c[i] > piv {
                let w = c[i]-piv;
                sum += w;
                target[i] = Some(w);
            }
        }
        if sum == 0 {
            return 0;
        }
        let bs = s.as_bytes();
        for i in 0..sum as usize {
            let pos = idx!(bs[i]);
            target[pos] = target[pos].map(|v| v-1);
        }
        let mut ans = n;
        let (mut l, mut r) = (0, sum as usize- 1);
        while r+1 < n {
            r += 1;
            let rch = bs[r];
            let pos = idx!(rch);
            target[pos] = target[pos].map(|v| v-1);
            let mut shrink = true;
            while shrink && l < r {
                shrink = false;
                let lch = bs[l];
                let pos = idx!(lch);
                match target[pos] {
                    None => {
                        l+=1;
                        shrink = true;
                    },
                    Some(v) if v < 0 => {
                        l += 1;
                        shrink = true;
                        target[pos] = Some(v+1);
                    },
                    _ => {},
                }
            }
            if Self::is_balance(&target) {
                ans = min(ans, r-l+1);
            }
        }
        ans as i32
    }
    fn is_balance(t: &[Option<i32>; 4]) -> bool {
        for i in 0..4 {
            if let Some(&v) = t[i].as_ref() {
                if v > 0 {
                    return false;
                }
            }
        }
        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_balanced_string() {
        let test_cases = vec![
            ("EQWEEEWERRWERQQQWWQEQWEEWQRWQWWWQWRWEWERWQEWWQWWQRRQWQERWWQRERRRRRWQEQRERRWRREEEERRWERQRQEWREQREWWEWRRRERWRRWEQWQQRRWQEREEEERWQWEWQEWRWREWQEREQWQEQWRQQQWRWWRWERWQWWQQREWREEWRWWQRWQRWWQWWREWWWEWQRWRRWQEWRRRWWQRRQREQRWRRQWREQWEQRWQRWQRWERRREEREEQQEREREWQQWRWEWEQQRWEWEQWEEQEEERWWWEQRRRWRQWWQQEQRWRWRWEQRRQRQRQWWWEQWERWEQRWQRERWQQQWWWQWRWEREWRQWQWERRQQWRQWRWQQQEEQREQWRWWEQRWWWEEERQWQWEWRQQRWQWEQQEWEWRQWWERERQREWWEEREQRRWQRRRWQEEWEWQEEEQRQQEWREQWQWRRWREQQQEEEWWRQEEEWQQEQEQWWREEWRQQQRQQEERQQQEEQWERRQEEQQRQQQWWEQEEQWRQWEQWEEQQEEQQEERQEEWQEEEWQRERRERQRQQQQWRQRRQQWQEEWRRQEWQWREWERWQRQQWEEERREQEEQWQQRWERWRWEEQWEREREWRWREEWRRRQEQQERRRERQRQWRWWREQQWWEQQWQWRRWEQRWEQQEQWWWQWWEWWEEEWEWRQWQWWQQQQEWRQEQWWRWWEQRRRWREREWRQQERQRQEQWWQQWRQRRQRWRQEEEWRRQREWRRERRQEEREERWEQQQWEWWEERWQWQQWWEQWWQWERRRWQWEEEEQEEWQRRWRWEEEWEWRREWWEQQERRQWQEWQQQRWEWRQEQWREWEWEEWWWWWWWWEQRWRRQQEEEERQEWQQEQQEEEWWRWRQQQQWRREEWRWWWQRWRWRWQWWEREQQEEEWREERQEWEERQWQREEQRQWEQEREQWWEWEEWREREWRQQWREEEEQRRWRWRQRRRWRQQQRRQQRQEWEEQQEREWQEEWEWWEQRRQWRQRQQREREQRWWQEEERREWRREWRRRQQQRQEQEWQWEEQQERQRRRWRRWWEEQRWQQQQQQWREWEWWRQREWREQRRQEQEERRQERWEEQRWWQRWQRWWERQEWEQERWRWQRQEEQEQEWWQWWRREEWEWWRRQEQQWEERWEWWWQEWWRRWRWQEERQRQWEQQEWWRRRRWQEWQQQQWWEEE", 16),
            ("RRRERWERREEEQEWWEEQERREWRWREWQQEQRRWRERRRWWREEEERRRQREWWQWEQERREERRQWEQQEQERWEEQWREEEQQEWRWRQRERWERR", 22),
            ("QWQQEQWQ", 4),
            ("QWQWEQQQ", 3),
            ("QWER", 0),
            ("QQWE", 1),
            ("QQQE", 2),
            ("QQQQ", 3),
        ];
        for (s, expect) in test_cases {
            assert_eq!(Solution::balanced_string(s.to_string()), expect, "s: {}", s);
        }

    }
}