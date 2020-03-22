impl Solution {
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let mut v: Vec<i32> = (lo..=hi).into_iter().collect();
        v.sort_by(|a,b| {
            let ta = Self::convert(*a);
            let tb = Self::convert(*b);
            if ta == tb {
                a.cmp(b)
            } else {
                ta.cmp(&tb)
            }
        });
        v[k as usize-1]
    }
    fn convert(mut v: i32) -> i32 {
        let mut ans = 0;
        while v > 1 {
            if v & 1 == 0 {
                v /= 2;
            } else {
                v = v*3+1;
            }
            ans += 1;
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_get_kth() {
        let test_cases = vec![
            (1, 1000, 777, 570),
        ];
        for (l,r,k,expect) in test_cases {
            assert_eq!(Solution::get_kth(l,r,k), expect, "l:{}, r:{}, k:{}",l,r,k);
        }
    }
}