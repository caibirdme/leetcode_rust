use std::collections::HashMap;

impl Solution {
    pub fn largest_number(cost: Vec<i32>, target: i32) -> String {
        let m = target as usize;
        let mut max_v: HashMap<i32, usize> = HashMap::new();
        for i in 0..9 {
            if let Some(idx) = max_v.get_mut(&cost[i]) {
                *idx = i.max(*idx);
            } else {
                max_v.insert(cost[i], i);
            }
        }
        let mut f = vec![-1; m+1];
        let mut select = vec![0; m+1];
        f[0] = 0;
        for i in 0..9 {
            let w = cost[i];
            if *max_v.get(&w).unwrap() != i {
                continue;
            }
            for v in w..=target {
                let pos = (v-w) as usize;
                let uv = v as usize;
                if f[pos] >= 0 {
                    let nval = f[pos]+1;
                    if nval > f[uv] {
                        f[uv] = nval;
                        select[uv] = i;
                    } else if nval == f[uv] {
                        select[uv] = i;
                    }
                }
            }
        }
        if f[m] <= 0 {
            return "0".to_owned();
        }
        let mut w = target;
        let mut ans = vec![];
        while w > 0 {
            let t = select[w as usize];
            ans.push((t as i32+1) as u8 + b'0');
            w -= cost[t];
        }
        unsafe {std::str::from_utf8_unchecked(&ans).to_string()}
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_largest_number() {
        let test_cases = vec![
            (vec![1,1,1,1,1,1,1,3,2], 10, "7777777777"),
            (vec![4,3,2,5,6,7,2,5,5], 9, "7772"),
            (vec![7,6,5,5,5,6,8,7,8], 12, "85"),
            (vec![2,4,6,2,4,6,4,4,4], 5, "0"),
            (vec![6,10,15,40,40,40,40,40,40], 47, "32211"),
        ];
        for (cost, target, expect) in test_cases {
            assert_eq!(Solution::largest_number(cost.clone(), target), expect, "cost: {:?}, target: {}", cost, target);
        }
    }
}