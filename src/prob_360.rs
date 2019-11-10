
impl Solution {
    pub fn sort_transformed_array(nums: Vec<i32>, a: i32, b: i32, c: i32) -> Vec<i32> {
        let f = |x:i32| a*x*x+b*x+c;
        if a == 0 {
            if b > 0 {
                return nums.iter().map(|&v| f(v)).collect();
            } else if b < 0{
                return nums.into_iter().rev().into_iter().map(|v| f(v)).collect();
            } else {
                return vec![c; nums.len()];
            }
        }
        let derivative = -b as f64 / (2*a) as f64;
        if a > 0 {
            if nums[0] as f64 >= derivative {
                return nums.into_iter().rev().into_iter().map(|v| f(v)).collect();
            } else if (*nums.last().unwrap()) as f64 <= derivative {
                return nums.iter().map(|&v| f(v)).collect();
            }
        } else if a < 0 {
            if nums[0] as f64 >= derivative {
                return nums.iter().map(|&v| f(v)).collect();
            } else if (*nums.last().unwrap()) as f64 <= derivative {
                return nums.into_iter().rev().into_iter().map(|v| f(v)).collect();
            }
            return Self::fast_path(nums, a,b,c);
        }
        let m = derivative.floor() as i32;
        let p = match nums.binary_search(&m) {
            Ok(idx) => {
                let d1 = (nums[idx] as f64 - derivative).abs();
                if idx+1 == nums.len() {
                    idx
                } else {
                    let d2 = (nums[idx+1] as f64 - derivative).abs();
                    if d2 < d1 {
                        idx+1
                    } else {
                        idx
                    }
                }
            },
            Err(idx) => {
                if idx == nums.len() {
                    idx
                } else {
                    if idx == 0 {
                        0
                    } else {
                        let d1 = (nums[idx] as f64 - derivative).abs();
                        let d2 = (nums[idx-1] as f64 - derivative).abs();
                        if d1 < d2 {
                            idx
                        } else {
                            idx-1
                        }
                    }
                }
            },
        };
        if p == 0 {
            return nums.iter().map(|&v| f(v)).collect();
        }
        if p == nums.len() {
            return nums.into_iter().rev().into_iter().map(|v| f(v)).collect();
        }
        let mut ans = vec![f(nums[p])];
        let mut l = Some(p-1);
        let mut r = p+1;
        while l.is_some() && r < nums.len() {
            let d1 = (nums[l.unwrap()] as f64 - derivative).abs();
            let d2 = (nums[r] as f64 - derivative).abs();
            if d1 < d2 {
                ans.push(f(nums[l.unwrap()]));
                l = l.and_then(|v| {
                    if v == 0 {
                        None
                    } else {
                        Some(v-1)
                    }
                });
            } else {
                ans.push(f(nums[r]));
                r+=1;
            }
        }
        while r < nums.len() {
            ans.push(f(nums[r]));
            r+=1;
        }
        while let Some(idx) = l.take() {
            ans.push(f(nums[idx]));
            if idx > 0 {
                l = Some(idx-1);
            } else {
                l = None;
            }
        }
        ans
    }
    fn fast_path(nums: Vec<i32>, a: i32, b: i32, c: i32) -> Vec<i32> {
        let mut l = 0;
        let mut r = nums.len()-1;
        let f = |x:i32| a*x*x+b*x+c;
        let mut ans = vec![];
        while l <= r {
            let y1 = f(nums[l]);
            let y2 = f(nums[r]);
            if y1 <= y2 {
                ans.push(y1);
                l+=1;
            } else {
                ans.push(y2);
                r-=1;
            }
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_sort_transformed_array() {
        let test_cases = vec![
            (vec![-3,-2,-1,0,1], 1,2,1, vec![0,1,1,4,4]),
            (vec![-4,-2,2,4], -1,3,5, vec![-23,-5,1,7]),
            (vec![-4,-2,2,4], 1,3,5, vec![3,9,15,33]),
        ];
        for (nums, a,b,c,expect) in test_cases {
            assert_eq!(Solution::sort_transformed_array(nums.clone(), a,b,c), expect, "nums: {:?}, a: {}, b: {}, c: {}", nums, a,b,c);
        }
    }
}