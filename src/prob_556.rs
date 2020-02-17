use std::cmp::Ordering;

impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut num = vec![];
        let mut t = n;
        while t > 0 {
            num.push(t%10);
            t /= 10;
        }
        let mut i = 0;
        while i+1 < num.len() && num[i] <= num[i+1] { i+=1; }
        if i == num.len()-1 {
            return -1;
        }
        let target = num[i+1]+1;
        let pos = match &num[..i+1].binary_search_by(|&probe| {
            if probe < target {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }) {
            Ok(pos) => *pos,
            Err(pos) => *pos,
        };
        num.swap(i+1, pos);
        let mut j = i;
        i = 0;
        while i < j {
            num.swap(i,j);
            i+=1;
            j-=1;
        }
        let mut ans = 0i64;
        for i in (0..num.len()).rev() {
            ans = ans * 10 + num[i] as i64;
        }
        if ans > std::i32::MAX as i64 {
            -1
        } else {
            ans as i32
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_next_greater_element() {
        let test_cases = vec![
            (1999999999, -1),
            (12222333, 12223233),
            (864453, 864534),
            (12,21),
            (21,-1),
            (586421, 612458),
            (87654321, -1),
            (12345678, 12345687),
        ];
        for (n, expect) in test_cases {
            assert_eq!(Solution::next_greater_element(n), expect, "n: {}", n);
        }
    }
}