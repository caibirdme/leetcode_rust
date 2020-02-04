impl Solution {
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        let can_reach = (max_choosable_integer+1)*max_choosable_integer/2;
        if can_reach < desired_total {
            return false;
        }
        if can_reach == desired_total {
            return max_choosable_integer&1 == 1;
        }
        let mut f = vec![None; 1<<(max_choosable_integer as usize)];
        Self::dfs(0, max_choosable_integer, desired_total, &mut f)
    }
    fn dfs(cur: i32, range: i32, target: i32, f: &mut Vec<Option<bool>>) -> bool {
        let ucur = cur as usize;
        if let Some(ok) = f[ucur] {
            return ok;
        }
        for i in (1..=range).rev() {
            let cur_bit = 1<<(i-1);
            if cur & cur_bit == 0 {
                if i >= target || !Self::dfs(cur | cur_bit, range, target-i, f) {
                    f[ucur] = Some(true);
                    return true;
                }
            }
        }
        f[ucur] = Some(false);
        false
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_can_i_win() {
        let test_cases = vec![
            (4, 8, true),
            (10, 11, false),
        ];
        for (range, target, ok) in test_cases {
            assert_eq!(Solution::can_i_win(range, target), ok, "range: {}, target: {}", range, target);
        }
    }
}