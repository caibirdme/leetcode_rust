impl Solution {
    pub fn read_binary_watch(num: i32) -> Vec<String> {
        if num == 0 {
            return vec!["0:00".to_string()];
        }
        let mut hour = vec![Vec::new(); 4];
        for i in 0..12 {
            hour[Self::count_one(i)].push(i.to_string());
        }
        let mut minute = vec![Vec::new(); 6];
        for i in 0..=59 {
            minute[Self::count_one(i)].push(Self::fmt(i));
        }
        let mut ans = vec![];
        for i in 0..=num {
            if i > 3 {
                break;
            }
            let j = num - i;
            if j > 5 {
                continue;
            }
            for h in hour[i as usize].iter() {
                for m in minute[j as usize].iter() {
                    ans.push(h.clone()+":"+m.as_str());
                }
            }
        }
        ans
    }

    fn fmt(i: i32) -> String {
        if i >= 10 {
            i.to_string()
        } else {
            "0".to_string() + i.to_string().as_str()
        }
    }

    fn count_one(mut n: i32) -> usize {
        let mut ans = 0;
        while n > 0 {
            ans += (n&1) as usize;
            n >>= 1;
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_read_binary_watch() {
        let test_cases = vec![
            (1, vec!["1:00", "2:00", "4:00", "8:00", "0:01", "0:02", "0:04", "0:08", "0:16", "0:32"]),
            (0, vec!["0:00"]),
        ];
        for (n, mut expect) in test_cases {
            expect.sort();
            let mut actual = Solution::read_binary_watch(n);
            actual.sort();
            assert_eq!(expect, actual, "n:{}",n);
        }
    }
}