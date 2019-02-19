impl Solution {
    pub fn diff_ways_to_compute(input: String) -> Vec<i32> {
        Self::diff_ways(input.as_bytes())
    }
    fn diff_ways(input: &[u8]) -> Vec<i32> {
        let n = input.len();
        if n == 0 {
            return vec![];
        }
        let mut res = vec![];
        for i in 0..n-1 {
            let ch = input[i];
            if Self::is_op(ch) {
                let (t1, t2) = (Self::diff_ways(&input[0..i]), Self::diff_ways(&input[i+1..]));
                for &x in t1.iter() {
                    for &y in t2.iter() {
                        res.push(Self::calc(x,y,ch));
                    }
                }
            }
        }
        use std::str;
        if res.is_empty() {
            vec![str::from_utf8(input).unwrap().parse::<i32>().unwrap()]
        } else {
            res
        }
    }
    fn is_op(c: u8) -> bool {
        match c {
            b'+'|b'-'|b'*' => true,
            _ => false,
        }
    }
    fn calc(a: i32, b:i32, t: u8) -> i32 {
        match t {
            b'+' => a + b,
            b'-' => a - b,
            b'*' => a * b,
            _ => unimplemented!(),
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_diff_ways_to_compute() {
        let mut res = Solution::diff_ways_to_compute("2-1-1".to_string());
        res.sort();
        assert_eq!(
            res,
            vec![0, 2]
        );
        res = Solution::diff_ways_to_compute("2*3-4*5".to_string());
        res.sort();
        assert_eq!(
            res,
            vec![-34, -14, -10, -10, 10]
        );
        assert_eq!(
            Solution::diff_ways_to_compute("100".to_string()),
            vec![100]
        );
        assert_eq!(
            Solution::diff_ways_to_compute("100*5".to_string()),
            vec![500]
        );
    }
}
