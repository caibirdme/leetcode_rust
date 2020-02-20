impl Solution {
    pub fn sum_nums(n: i32) -> i32 {
        let mut ans = 0;
        Self::add(n, &mut ans);
        ans
    }
    fn add(i: i32, ans: &mut ans) -> bool {
        *ans += i;
        return i>0 && Self::add(i-1, ans)
    }
}

struct Solution;