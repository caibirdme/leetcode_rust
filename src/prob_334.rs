impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n < 3 {
            return false;
        }
        let mut least = nums[0];
        let mut opt_least: Option<i32> = None;
        let mut mid: Option<i32> = None;
        for i in 1..n {
            let t= nums[i];
            if t < least {
                if mid.is_none() {
                    least = nums[i];
                } else {
                    match opt_least {
                        Some(v) => {
                            if t<v {
                                opt_least = Some(t);
                            } else {
                                opt_least = None;
                                least = v;
                                mid = Some(t);
                            }
                        },
                        None => {
                            opt_least = Some(t);
                        },
                    }
                }
            } else if t == least {
                match opt_least {
                    Some(v) if v < t => {
                        least = v;
                        mid = Some(t);
                        opt_least = None;
                    },
                    _ => {},
                }
            } else {
                match mid {
                    Some(m) if t>m => {
                        return true;
                    },
                    _ => {
                        mid = Some(t);
                    },
                }
            }
        }
        false
    }
    pub fn increasing_triplet_2(nums: Vec<i32>) -> bool {
        let (mut low, mut high) = (i32::max_value(), i32::max_value());
        for num in nums {
            if num <= low {
                low = num;
            } else if num <= high {
                high = num;
            } else {
                return true
            }
        }

        false
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_increasing_triplet() {
        let test_cases = vec![
            (vec![3,4,2,5], true),
            (vec![1,1,-2,6], false),
            (vec![1,2,3], true),
            (vec![1,2,2], false),
            (vec![3,2,2], false),
            (vec![1,2,3,4,5], true),
            (vec![5,4,3,2,1], false),
            (vec![1,9,8,7,6,5,4,2,3], true),
            (vec![3,4,2,1,5,2,1,1,2,2,2,2,2,2,2], true),
        ];
        for (nums, ok) in test_cases {
            assert_eq!(
                Solution::increasing_triplet(nums),
                ok,
            );
        }
    }
    #[test]
    fn test_increasing_triplet_2() {
        let test_cases = vec![
            (vec![3,4,2,5], true),
            (vec![1,1,-2,6], false),
            (vec![1,2,3], true),
            (vec![1,2,2], false),
            (vec![3,2,2], false),
            (vec![1,2,3,4,5], true),
            (vec![5,4,3,2,1], false),
            (vec![1,9,8,7,6,5,4,2,3], true),
            (vec![3,4,2,1,5,2,1,1,2,2,2,2,2,2,2], true),
        ];
        for (nums, ok) in test_cases {
            assert_eq!(
                Solution::increasing_triplet_2(nums),
                ok,
            );
        }
    }
}