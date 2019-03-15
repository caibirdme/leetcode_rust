use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn can_measure_water(x: i32, y: i32, z: i32) -> bool {
        if x < y {
            return Self::can_measure_water(y,x,z);
        }
        if z == x || z ==y {
            return true;
        }
        if x+y == z {
            return true;
        }
        if x+y < z {
            return false;
        }
        if x == 0 {
            return y == z;
        }
        if y == 0 {
            return x == z;
        }
        z % Self::gcd(x,y) == 0
    }
    pub fn gcd(mut a: i32,mut b: i32) -> i32 {
        while b > 0 {
            let t = a%b;
            a = b;
            b = t;
        }
        a
    }

    pub fn can_measure_water_dfs(x: i32, y: i32, z: i32) -> bool {
        if z == x || z ==y {
            return true;
        }
        if x+y == z {
            return true;
        }
        if x+y < z {
            return false;
        }
        if x == 0 {
            return y == z;
        }
        if y == 0 {
            return x == z;
        }
        let mut f = HashMap::new();
        let mut ok = false;
        Self::dfs(0,0,x as usize, y as usize, z as usize, &mut f, &mut ok);
        ok
    }
    fn dfs(a: usize, b: usize, x: usize, y: usize, z: usize, f: &mut HashMap<usize, HashSet<usize>>, ok: &mut bool) {
        if *ok {
            return;
        }
        match f.get(&a) {
            Some(v) => {
                if v.contains(&b) {
                    return;
                }
            },
            None => {},
        }
        if a == z || b == z || a+b == z{
            *ok = true;
            return;
        }
        f.entry(a).or_insert(HashSet::new()).insert(b);
        // 把a装满
        if a != x {
            Self::dfs(x,b,x,y,z,f,ok);
        }
        // 把b装满
        if b != y {
            Self::dfs(a,y,x,y,z,f,ok);
        }
        // 把a倒到b
        if a > 0 && b < y {
            let t = b+a;
            if t > y {
                Self::dfs(t-y, y,x,y,z,f,ok);
            } else {
                Self::dfs(0,t,x,y,z,f,ok);
            }
        }
        // 把b清空把a倒到b
        if a > 0 {
            if a > y {
                Self::dfs(0,y,x,y,z,f,ok);
            } else {
                Self::dfs(0,a,x,y,z,f,ok);
            }
        }
        // 把b倒到a
        if b > 0 && a < x{
            let t = b+a;
            if t > x {
                Self::dfs(x, t-x,x,y,z,f,ok);
            } else {
                Self::dfs(t,0,x,y,z,f,ok);
            }
        }
        // 把a清空把b倒入a
        if b > 0 {
            if b > x {
                Self::dfs(x,0,x,y,z,f,ok);
            } else {
                Self::dfs(b,0,x,y,z,f,ok);
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_gcd() {
        let test_cases = vec![
            (10,5,5),
            (8,6,2),
            (32,24,8),
        ];
        for (a,b,c) in test_cases {
            assert_eq!(c, Solution::gcd(a,b), "a:{}, b:{}",a,b);
        }
    }
    #[test]
    fn test_can_measure_water() {
        let test_cases = vec![
            (104597, 104623, 123, true),
            (3,5,4, true),
            (2,6,5,false),
            (1,0,0, true),
        ];
        for (x,y,z,ok) in test_cases {
            assert_eq!(ok,Solution::can_measure_water(x,y,z));
        }
    }
}