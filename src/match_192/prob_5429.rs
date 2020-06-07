impl Solution {
    pub fn get_strongest(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
        arr.sort();
        let n = arr.len();
        let m = arr[(n-1)/2];
        arr.sort_by(|&a,&b| {
            let absa = Self::abs(a-m);
            let absb = Self::abs(b-m);
            if absa == absb {
                a.cmp(&b)
            } else {
                absa.cmp(&absb)
            }
        });
        let mut ans = vec![];
        for i in (n-(k as usize)..n).rev() {
            ans.push(arr[i]);
        }
        ans
    }
    fn abs(x: i32) -> i32 {
        if x < 0 {
            -x
        } else {
            x
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_get_strongest() {
        let test_cases = vec![
            (vec![1,2,3,4,5], 2, vec![5,1]),
            (vec![1,1,3,5,5], 2, vec![5,5]),
            (vec![6,7,11,7,6,8], 5, vec![11,8,6,6,7]),
        ];
        for (nums, k, expect) in test_cases {
            assert_eq!(Solution::get_strongest(nums.clone(), k), expect, "nums:{:?} k: {}", nums, k);
        }
    }
}