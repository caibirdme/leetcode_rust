impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        if n == 0 {
            return vec![];
        }
        let mut ans = vec![0; n as usize];
        let mut stack: Vec<usize> = vec![];
        let mut cur_time = 0;
        for log in &logs {
            let (id, is_start, timestamp) = Self::split(log.as_bytes());
            if is_start {
                if let Some(id) = stack.last() {
                    ans[*id] += timestamp - cur_time;
                }
                stack.push(id);
                cur_time = timestamp;
            } else {
                let id = stack.pop().unwrap();
                ans[id] += timestamp - cur_time + 1;
                cur_time = timestamp+1;
            }
        }
        ans
    }
    fn split(s: &[u8]) -> (usize, bool, i32) {
        let mut id = 0;
        let mut i = 0;
        while s[i] != b':' {
            id = id*10 + (s[i]-b'0') as i32;
            i+=1;
        }
        i+=1;
        let ok = if s[i] == b's' {
            i += 6;
            true
        } else {
            i += 4;
            false
        };
        let mut timestamp = 0;
        while i < s.len() {
            timestamp = timestamp * 10 + (s[i]-b'0') as i32;
            i+=1;
        }
        (id as usize, ok, timestamp)
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_exclusive_time() {
        let test_cases = vec![
            (
                3,
                vec!["0:start:0",
                     "1:start:2",
                     "1:end:5",
                     "2:start:6",
                     "2:end:8",
                     "0:end:9",
                ],
                vec![3,4,3],
            ),
            (
                2,
                vec!["0:start:0",
                  "1:start:2",
                  "1:end:5",
                  "0:end:6",
                ],
                vec![3,4],
            ),
        ];
        for (n, logs, expect) in test_cases {
            assert_eq!(Solution::exclusive_time(n, logs.iter().map(|v| v.to_string()).collect()), expect, "n:{}, logs: {:?}", n, logs);
        }
    }

}