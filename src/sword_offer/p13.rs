use std::collections::HashSet;

impl Solution {
    pub fn moving_count(m: i32, n: i32, k: i32) -> i32 {
        let mut went = HashSet::new();
        let mut ans = 0;
        let mut q = vec![(0,0)];
        went.insert((0,0));
        let mut head = 0;
        let step = [(0,1),(1,0),(0,-1),(-1,0)];
        while head < q.len() {
            let (x,y) = q[head];
            ans += 1;
            head += 1;
            for (dx,dy) in &step {
                let (nx,ny) = (x+*dx,y+*dy);
                if nx < 0 || nx >= m || ny < 0 || ny >= n || went.contains(&(nx,ny)){
                    continue;
                }
                if !Self::can_go(nx,ny,k) {
                    continue;
                }
                went.insert((nx,ny));
                q.push((nx,ny));
            }
        }
        ans
    }
    fn can_go(x: i32, y: i32, k: i32) -> bool {
        if Self::add(x)+Self::add(y) <= k {
            return true;
        }
        false
    }
    fn add(mut x: i32) -> i32 {
        let mut count = 0;
        while x > 0 {
            count += x % 10;
            x /= 10;
        }
        count
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_moving_count() {
        let test_cases = vec![
            (2,3,1,3),
            (3,1,0,1),
        ];
        for (m,n,k,expect) in test_cases {
            assert_eq!(Solution::moving_count(m,n,k),expect,"m:{},n:{},k:{}",m,n,k);
        }
    }
}