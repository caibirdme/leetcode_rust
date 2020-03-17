impl Solution {
    pub fn beautiful_array(n: i32) -> Vec<i32> {
        if n == 1 {
            return vec![1];
        }
        if n == 2 {
            return vec![1, 2];
        }
        // 只有最多2*logN次递归, 不需要记忆化
        let (mut left, mut right) = if n & 1 == 0 {
            let t = Self::beautiful_array(n/2);
            (t.clone(), t)
        } else {
            (Self::beautiful_array(n/2+1), Self::beautiful_array(n/2))
        };
        left.iter_mut().for_each(|v| *v = (*v) * 2 -1);
        right.iter_mut().for_each(|v| *v *= 2);
        left.append(&mut right);
        left
    }
}

struct Solution;