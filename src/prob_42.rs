impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut i = 0;
        while i+1 < n && height[i] <= height[i+1] {i+=1;}
        let mut j = n-1;
        while j > i && height[j] <= height[j-1] {j-=1;}
        let mut ans = 0;
        while i < j {
            if height[i] <= height[j] {
                let left = height[i];
                while i < j && height[i+1] <= left {
                    i += 1;
                    ans += left-height[i];
                }
                i+=1;
            } else {
                let right = height[j];
                while j > i && height[j-1] <= right {
                    j -= 1;
                    ans += right - height[j];
                }
                j -= 1;
            }
        }
        ans
    }
}

struct Solution;