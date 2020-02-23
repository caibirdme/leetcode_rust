impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let n = num_rows as usize;
        let mut ans = vec![vec![]; n];
        let mut iter = Pointer::new(num_rows);
        for &c in s.as_bytes() {
            let i = iter.next().unwrap();
            ans[i].push(c);
        }
        ans.into_iter().map(|v| unsafe{std::str::from_utf8_unchecked(&v).to_string()}).collect::<Vec<String>>().join("")
    }
}

struct Pointer {
    i: usize,
    rows: usize,
    add: bool,
}

impl Pointer {
    fn new(rows: i32) -> Self {
        Self{
            i:0,
            rows: rows as usize-1,
            add: true,
        }
    }
}

impl Iterator for Pointer {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let ans = self.i;
        if self.i == self.rows {
            self.add = false;
        } else if self.i == 0 {
            self.add = true;
        }
        if self.add {
            self.i += 1;
        } else {
            self.i -= 1;
        }
        Some(ans)
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_convert() {
        let test_cases = vec![
            ("PAYPALISHIRING", 3, "PAHNAPLSIIGYIR"),
        ];
        for (s, rows, expect) in test_cases {
            assert_eq!(Solution::convert(s.to_string(), rows), expect, "s: {}, rows: {}", s, rows);
        }
    }
}