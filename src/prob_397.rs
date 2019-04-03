//给定一个正整数 n，你可以做如下操作：
//
// 1. 如果 n 是偶数，则用 n / 2替换 n。
//2. 如果 n 是奇数，则可以用 n + 1或n - 1替换 n。
//n 变为 1 所需的最小替换次数是多少？
//
// 示例 1:
//
//
//输入:
//8
//
//输出:
//3
//
//解释:
//8 -> 4 -> 2 -> 1
//
//
// 示例 2:
//
//
//输入:
//7
//
//输出:
//4
//
//解释:
//7 -> 8 -> 4 -> 2 -> 1
//或
//7 -> 6 -> 3 -> 2 -> 1
//
//

impl Solution {
    pub fn integer_replacement(mut n: i32) -> i32 {
        let mut ans = 0;
        if n < 2 {
            return 0;
        }
        if n == 2 {
            return 1;
        }
        while n&1 == 0 && n>1 {
            n /= 2;
            ans += 1;
        }
        if n > 1 {
            use std::cmp::min;
            let t = n/2;
            ans + min(Self::integer_replacement(t), Self::integer_replacement(t+1)) + 2
        } else {
            ans
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_integer_replacement() {
        let test_cases = vec![
            (20000000,31),
            (8,3),
            (7,4),
            (2,1),
            (1,0),
        ];
        for (n,e) in test_cases {
            assert_eq!(e, Solution::integer_replacement(n), "n: {}", n);
        }
    }
}