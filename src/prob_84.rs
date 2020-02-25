impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        if heights.is_empty() {
            return 0;
        }
        let mut q = vec![0usize];
        let mut ans = 0;
        for i in 1..heights.len() {
            while let Some(&idx) = q.last() {
                if heights[idx] >= heights[i] {
                    q.pop();
                    if q.is_empty() {
                        ans = ans.max(heights[idx]*(i as i32));
                    } else {
                        ans = ans.max((i-*q.last().unwrap()-1) as i32* heights[idx]);
                    }
                } else {
                    break;
                }
            }
            q.push(i);
        }
        let mut j = heights.len()-1;
        while let Some(idx) = q.pop() {
            if q.is_empty() {
                ans = ans.max((j+1) as i32 * heights[idx]);
            } else {
                ans = ans.max((j-*q.last().unwrap()) as i32 * heights[idx]);
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
    fn test_largest_rectangle_area() {
        let test_cases = vec![
            (vec![4,2], 4),
            (vec![2,1,5,6,2,3], 10),
        ];
        for (heights, expect) in test_cases {
            assert_eq!(Solution::largest_rectangle_area(heights.clone()), expect, "heights: {:?}", heights);
        }
    }
}