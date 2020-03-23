impl Solution {
    pub fn min_domino_rotations(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let n = a.len() as i32;
        let mut ca = [0; 7];
        let mut cb = [0; 7];
        let mut double = [0; 7];
        for (va,vb) in a.into_iter().zip(b.into_iter()) {
            ca[va as usize] += 1;
            cb[vb as usize] += 1;
            if va == vb {
                double[va as usize] += 1;
            }
        }
        let mut ans = std::i32::MAX;
        for i in 1..=6 {
            if ca[i]+cb[i]-double[i] >= n {
                ans = ans.min(ca[i].min(cb[i])-double[i]);
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
    fn test_min_domino_rotations() {
        let test_cases = vec![
            (vec![1,2,1,1,1,2,2,2], vec![2,1,2,2,2,2,2,2], 1),
            (vec![2,2,3,3], vec![1,3,2,2], 2),
            (vec![2,1,2,4,2,2], vec![5,2,6,2,3,2], 2),
            (vec![3,5,1,2,3], vec![3,6,3,3,4], -1),
        ];
        for (a, b, expect) in test_cases {
            assert_eq!(Solution::min_domino_rotations(a.clone(), b.clone()), expect, "a: {:?}, b: {:?}", a,b);
        }
    }
}