use std::collections::HashSet;

impl Solution {
    pub fn simplified_fractions(n: i32) -> Vec<String> {
        if n == 1 {
            return vec![];
        }
        let mut hash = HashSet::new();
        for i in 2..=n {
            for j in 1..i {
                let g = Self::gcd(i, j);
                hash.insert((i/g, j/g));
            }
        }
        hash.into_iter().map(|(x,y)| format!("{}/{}",y,x)).collect()
    }
    fn gcd(mut x: i32, mut y:i32) -> i32 {
        if x < y {
            return Self::gcd(y, x);
        }
        while x%y != 0 {
            let m = x%y;
            x = y;
            y = m;
        }
        y
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_simplified_fractions() {
        let test_cases = vec![
            (2, vec!["1/2"]),
            //(3, vec!["1/2","1/3","2/3"]),
            (4, vec!["1/2","1/3","1/4","2/3","3/4"]),
        ];
        for (n, expect) in test_cases {
            assert_eq!(Solution::simplified_fractions(n), expect);
        }
    }
}