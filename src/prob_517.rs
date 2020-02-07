macro_rules! abs {
    ($e: ident) => {
        if $e > 0 { $e } else { -$e }
    };
}

impl Solution {
    /*
    先算出平均值，高于平均值的表面要移除，少于平均值的表示要移入。把每个数减去平均数得到第i个洗衣机要移除或移入多少
    如果第一个洗衣机为+x，表示它无论如何要x轮才能把自己变到平均数
    如果第二个数依然为+y，表示它不论如何也至少移y轮。但是由于第一个数也要移出，它们都只能向右移，每次只能移动1个，且累计多了x+y个了。
    因此对于前两个洗衣机，至少要移动x+y才行。
    如果第三个数为-z，表示至少接收z个。前三个总计x+y-z=t，表示要把前三个都移动成平均数，至少还要需要接收或者移出t个。
    每次找最大值即可。
    */
    pub fn find_min_moves(mut machines: Vec<i32>) -> i32 {
        let n = machines.len();
        if n == 0 {
            return 0;
        }
        let total: i32 = machines.iter().sum();
        if total % (n as i32) != 0  {
            return -1;
        }
        let target = total / (n as i32);
        machines.iter_mut().for_each(|v| *v -= target);
        let mut sum = 0;
        let mut ans = 0;
        for &v in &machines {
            sum += v;
            ans = ans.max(abs!(sum));
            ans = ans.max(v);
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_min_moves() {
        let test_cases = vec![
            (vec![5,2,5,5,2,5,5,2,5,5,2,5], 1),
            (vec![1,0,5], 3),
            (vec![0,3,0], 2),
            (vec![0,2,0], -1),
        ];
        for (machines, expect) in test_cases {
            assert_eq!(Solution::find_min_moves(machines.clone()), expect, "machines: {:?}", machines);
        }
    }
}