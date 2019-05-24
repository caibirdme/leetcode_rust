impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut stack = vec![];
        let mut sj: i32 = std::i32::MIN;
        for &i in nums.iter().rev() {
            if stack.is_empty() {
                stack.push(i);
                continue;
            }
            if i < sj {
                return true;
            }
            while !stack.is_empty() {
                let &last = stack.last().unwrap();
                if i == last {
                    break;
                }
                if i < last {
                    stack.push(i);
                    break;
                }
                sj = stack.pop().unwrap();
            }
            if stack.is_empty() {
                stack.push(i);
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
    fn test_find132pattern() {
        let test_cases = vec![
            (vec![3,100,3,3,2,4,1,1], true),
            (vec![3,100,3,3,2,2,1,1], false),
            (vec![-2,1,1], false),
            (vec![3,1,4,2], true),
            (vec![1,2,3,4], false),
            (vec![-1,3,2,0], true),
        ];
        for (nums, ok) in test_cases {
            assert_eq!(ok, Solution::find132pattern(nums.clone()), "nums {:?}", nums);
        }
    }
}