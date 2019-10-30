#[derive(Copy, Clone)]
struct State(i32, i32);

impl State {
    fn new(x: i32, y: i32) -> Self {
        let p = Self::convert(x,y);
        Self(1<<p, p)
    }
    fn convert(x: i32, y: i32) -> i32 {
        match (x,y) {
            (0,0) => 1,
            (0,1) => 2,
            (0,2) => 3,
            (1,0) => 4,
            (1,1) => 5,
            (1,2) => 6,
            (2,0) => 7,
            (2,1) => 8,
            (2,2) => 9,
            _ => unreachable!()
        }
    }
    fn pos_to_loc(p: i32) -> (i32, i32) {
        match p {
            1 => (0,0),
            2 => (0,1),
            3 => (0,2),
            4 => (1,0),
            5 => (1,1),
            6 => (1,2),
            7 => (2,0),
            8 => (2,1),
            9 => (2,2),
            _ => unreachable!(),
        }
    }
    fn get_loc(&self) -> (i32, i32) {
        Self::pos_to_loc(self.1)
    }
    fn set_as_visited(&self, x: i32, y: i32) -> Self {
        let p = Self::convert(x,y);
        Self(self.0 | (1<<p), p)
    }
    fn is_visited(&self, x: i32, y: i32) -> bool {
        let p = Self::convert(x,y);
        let t = 1<<p;
        self.0 & t == t
    }
}

impl Solution {
    pub fn number_of_patterns(m: i32, n: i32) -> i32 {
        if m > n {
            return 0;
        }
        let a = 4*Self::bfs(0,0,m,n);
        let b = 4*Self::bfs(1,0,m,n);
        let c = Self::bfs(1,1,m,n);
        a+b+c
    }
    fn bfs(xx: i32, yy: i32, m: i32, n: i32) -> i32 {
        let steps = [(0,1),(0,-1),(1,0),(-1,0),(1,1),(1,-1),(-1,1),(-1,-1),(1,2),(1,-2),(-1,2),(-1,-2),(2,1),(2,-1),(-2,1),(-2,-1)];
        let steps_line = [(2,0),(0,2),(-2,0),(0,-2),(2,2),(2,-2),(-2,2),(-2,-2)];
        let mut count = 0;
        let mut q = vec![(State::new(xx,yy), 1)];
        let mut head = 0;
        while head < q.len() {
            let (state, step) = q[head];
            head += 1;
            if step >= m {
                count += 1;
            }
            if step >= n {
                continue;
            }
            let (x,y) = state.get_loc();
            for (dx,dy) in steps.iter() {
                let nx = x+*dx;
                let ny = y+*dy;
                if nx < 0 || nx > 2 || ny < 0 || ny > 2 {
                    continue;
                }
                if !state.is_visited(nx,ny) {
                    q.push((state.set_as_visited(nx,ny), step+1));
                }
            }
            for (dx,dy) in steps_line.iter() {
                let nx = x+*dx;
                let ny = y+*dy;
                if nx < 0 || nx > 2 || ny < 0 || ny > 2 {
                    continue;
                }
                let mx = x + *dx/2;
                let my = y + *dy/2;
                if !state.is_visited(nx,ny) && state.is_visited(mx,my) {
                    q.push((state.set_as_visited(nx,ny), step+1));
                }
            }
        }
        count
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_number_of_patterns() {
        assert_eq!(1<<0, 1);
        let test_cases = vec![
            (3,3,320),
            (2,2,56),
            (1,1,9),
        ];
        for (m,n,expect) in test_cases {
            assert_eq!(Solution::number_of_patterns(m,n), expect, "m: {}. n: {}",m,n);
        }
    }
}