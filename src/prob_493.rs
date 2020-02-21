use std::cmp::Ordering;

impl Solution {
    pub fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return 0;
        }
        let mut ans = 0;
        Self::merge_sort(&mut nums, 0, n-1, &mut ans);
        ans as i32
    }
    fn merge_sort(nums: &mut Vec<i32>, l: usize, r: usize, ans: &mut usize) {
        static BOUND: i32 = (std::i32::MAX-1)/2;
        if l >= r { return ; }
        let mid = (l+r)/2;
        Self::merge_sort(nums, l, mid, ans);
        Self::merge_sort(nums, mid+1, r, ans);
        let n = r-l+1;
        let mut tmp = Vec::with_capacity(n);
        let mut i = l;
        let mut j = mid+1;
        while i <= mid && j <= r {
            if nums[i] < nums[j] {
                if nums[j] < 0 {
                    let q = nums[i]/2-1;
                    if let Err(pos) = nums[j..r+1].binary_search_by(|&probe| {
                        if probe <= q {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    }) {
                        *ans += pos;
                    }
                }
                tmp.push(nums[i]);
                i+=1;
            } else {
                let q = nums[j] as i64 * 2+1;
                if let Err(pos) = nums[i..mid+1].binary_search_by(|&probe| {
                    if (probe as i64) < q {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }) {
                    *ans += mid+1-i-pos;
                }
                tmp.push(nums[j]);
                j+=1;
            }
        }
        while i <= mid {tmp.push(nums[i]); i+=1;}
        while j <= r {tmp.push(nums[j]); j+=1;}
        for i in 0..n {
            nums[l+i] = tmp[i];
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let v = vec![1,2,3,3,3];
        let t = v.binary_search_by(|&probe| {
            if (probe as i64) < 3 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        assert_eq!(t, Err(2));
    }

    #[test]
    fn test_reverse_pairs() {
        let test_cases = vec![
            (vec![-100,-90,-80,-51,-50,1], 9),
            (vec![233,2000000001,234,2000000006,235,2000000003,236,2000000007,237,2000000002,2000000005,233,233,233,233,233,2000000004], 40),
        ];
        for (nums, expect) in test_cases {
            assert_eq!(Solution::reverse_pairs(nums.clone()), expect, "nums: {:?}", nums);
        }
    }
}