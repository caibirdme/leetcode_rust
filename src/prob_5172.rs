impl Solution {
    pub fn largest_multiple_of_three(digits: Vec<i32>) -> String {
        let mut count = [0; 10];
        for v in digits {
            count[v as usize] += 1;
        }
        let mut found = [0; 10];
        let mod_2 = count[8]+count[5]+count[2];
        let mod_1 = count[7]+count[4]+count[1];
        if mod_1==mod_2 {
            found = count;
        } else {
            let mut f = vec![vec![-1; mod_1+1]; mod_2+1];
            let mut record = vec![vec![0; mod_1+1]; mod_2+1];
            let total = Self::dfs(mod_2, mod_1, &mut f, &mut record);
            let mut x = 0;
            let mut y = 0;
            let mut i = mod_2;
            let mut j = mod_1;
            while i > 0 && j > 0 {
                let w = record[i][j];
                if w == 1 {
                    x += 1;
                    y += 1;
                    i-=1;
                    j-=1;
                } else if w == 2 {
                    x += 3;
                    i-=3;
                } else {
                    y += 3;
                    j-=3;
                }
            }
            if i > 0 {
                x += i / 3 * 3;
            } else {
                y += j / 3 * 3;
            }
            for &v in [8,5,2].iter() {
                let uv = v as usize;
                if count[uv] == 0 {
                    continue;
                }
                if count[uv] <= x {
                    found[uv] = count[uv];
                    x -= count[uv];
                } else {
                    found[uv] = x;
                    break;
                }
            }
            for &v in [7,4,1].iter() {
                let uv = v as usize;
                if count[uv] == 0 {continue;}
                if count[uv] <= y {
                    found[uv] = count[uv];
                    y -= count[uv];
                } else {
                    found[uv] = y;
                    break;
                }
            }
            found[0] = count[0];
            found[3] = count[3];
            found[6] = count[6];
            found[9] = count[9];
        }
        let mut ans = vec![];
        for i in (0..10).rev() {
            if found[i] > 0 {
                for j in 0..found[i] {
                    ans.push(i as u8+b'0');
                }
            }
        }
        if ans.is_empty() {
            "".to_string()
        } else if ans[0] == b'0' {
            "0".to_string()
        } else {
            unsafe {std::str::from_utf8_unchecked(&ans).to_string()}
        }
    }
    fn dfs(x: usize, y: usize, f: &mut Vec<Vec<i32>>, record: &mut Vec<Vec<i32>>) -> i32 {
        if x == 0 {
            return (y/3*3) as i32;
        }
        if y == 0 {
            return (x/3*3) as i32;
        }
        if f[x][y] != -1 {
            return f[x][y];
        }
        let mut t = Self::dfs(x-1, y-1, f, record) + 2;
        record[x][y] = 1;
        if x >= 3 {
            let q = Self::dfs(x-3, y, f,record)+3;
            if q > t {
                t = q;
                record[x][y] = 2;
            }
        }
        if y >= 3 {
            let q = Self::dfs(x, y-3, f,record)+3;
            if q > t {
                t = q;
                record[x][y] = 3;
            }
        }
        f[x][y] = t;
        t
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_largest_multiple_of_three() {
        let test_cases = vec![
            (vec![8,6,0,7,1], "8760"),
            (vec![2,2,1,1,1], "2211"),
            (vec![9,9,6,6,3,8,8,7,5,5,2,2], "99886655322"),
            (vec![2,1,1,1], "111"),
            (vec![8,1,9], "981"),
            (vec![1,1], ""),
            (vec![0,0,0,0,0,0,0], "0"),
        ];
        for (nums, expect) in test_cases {
            assert_eq!(Solution::largest_multiple_of_three(nums.clone()), expect, "nums: {:?}", nums);
        }
    }
}