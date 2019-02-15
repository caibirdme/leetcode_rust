impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut x = 1e8 as i32;
        let mut y  = x;
        let mut count_x = 0;
        let mut count_y = 0;
        for &i in nums.iter() {
            if i == x {
                count_x += 1;
            } else if i ==y {
                count_y += 1;
            } else if count_x == 0 {
                x = i;
                count_x = 1;
            } else if count_y == 0 {
                y = i;
                count_y = 1;
            } else {
                count_x -= 1;
                count_y -= 1;
            }
        }
        count_x = 0;
        count_y = 0;
        for &i in nums.iter() {
            if i == x {
                count_x += 1;
            } else if i == y {
                count_y += 1;
            }
        }
        let mut res = Vec::with_capacity(2);
        if count_x > nums.len() / 3 {
            res.push(x);
        }
        if count_y > nums.len() / 3 {
            res.push(y);
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_majority_element() {
        assert_eq!(
            Solution::majority_element(vec![3,2,3]),
            vec![3]
        );
        assert_eq!(
            Solution::majority_element(vec![1,1,1,3,3,2,2,2]),
            vec![1,2]
        );
    }
}