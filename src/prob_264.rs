
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        let mut nums = vec![1];
        let (mut p2, mut p3, mut p5) = (0,0,0);
        for _ in 1..n {
            let v2 = nums[p2]*2;
            let v3 = nums[p3]*3;
            let v5 = nums[p5]*5;
            let m = v2.min(v3.min(v5));
            if m == v2 {
                p2+=1;
            }
            if m == v3 {
                p3 += 1;
            }
            if m == v5 {
                p5 += 1;
            }
            nums.push(m);
        }
        *nums.last().unwrap()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_nth_ugly_number() {
        assert_eq!(
            Solution::nth_ugly_number(10),
            12
        );
        assert_eq!(
            Solution::nth_ugly_number(1),
            1
        );
        assert_eq!(
            Solution::nth_ugly_number(2),
            2
        );
        assert_eq!(
            Solution::nth_ugly_number(4),
            4
        );
        assert_eq!(
            Solution::nth_ugly_number(9),
            10
        );
        assert_eq!(
            Solution::nth_ugly_number(1407),
            536870912
        );
    }
}