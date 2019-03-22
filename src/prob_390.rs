impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        Self::L(n)
    }
    fn L(n: i32) -> i32 {
        if n == 1 {
            1
        } else {
            2*Self::R(n/2)
        }
    }
    fn R(n: i32) -> i32 {
        if n == 1 {
            1
        } else if n&1==1 {
            2*Self::L(n/2)
        } else {
            2*Self::L(n/2) -1
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_last_remaining() {
        let test_cases = vec![
            (1,1),
            (2,2),
            (3,2),
            (4,2),
            (5,2),
            (6,4),
            (7,4),
            (8,6),
            (9,6),
            (10,8),
            (11,8),
        ];
        for (n,expect) in test_cases{
            assert_eq!(expect, Solution::last_remaining(n), "n: {}",n)
        }
    }
}