impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let n = arr.len();
        let mut t = 0;
        let mut vv = vec![];
        while r < n {
            while r < n {
                t += arr[r];
                r += 1;
                if t > target {
                    break;
                } else if t == target{
                    vv.push((r-1, r-l));
                }
            }
            while t > target {
                t -= arr[l];
                l += 1;
            }
            if t == target {
                vv.push((r-1, r-l));
            }
        }
        if vv.len() < 2 {
            return -1;
        }
        l = 0;
        r = vv.len()-1;
        let mut ans = std::i32::MAX;
        let mut f = vec![0; vv.len()];
        let mut pre = std::i32::MAX;
        for i in 0..vv.len() {
            f[i] = pre.min(vv[i].1 as i32);
            pre = pre.min(f[i]);
        }
        for i in (0..vv.len()).rev() {
            let idx = vv[i].0+1-vv[i].1;
            for j in (0..i).rev() {
                if vv[j].0 < idx {
                    ans = ans.min(vv[i].1 as i32 + f[j]);
                    break;
                }
            }
        }
        if ans == std::i32::MAX {
            -1
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
    fn test_min_sum_of_lengths() {
        let test_cases = vec![
            (vec![31,1,1,18,15,3,15,14],33,5),
            (vec![7,3,4,7], 7, 2),
            (vec![3,2,2,4,3], 3, 2),
            (vec![4,3,2,6,2,3,4], 6, -1),
            (vec![5,5,4,4,5], 3, -1),
            (vec![3,1,1,1,5,1,2,1], 3, 3),
        ];
        for (arr, target, expect) in test_cases {
            assert_eq!(Solution::min_sum_of_lengths(arr.clone(), target), expect, "arr:{:?}, target: {}", arr, target);
        }
    }
}