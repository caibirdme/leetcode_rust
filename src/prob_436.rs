struct PosInterval {
    v: Vec<i32>,
    p: usize
}

impl PosInterval {
    #[inline]
    pub fn new(v: Vec<i32>, p: usize) -> Self {
        Self{v,p,}
    }
}

impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        use std::cmp::Ordering;
        let n = intervals.len();
        if n == 0 {
            return vec![];
        }
        if n == 1 {
            return vec![-1];
        }
        let mut pvs: Vec<PosInterval> = intervals
            .into_iter().enumerate()
            .map(|(i,v)| PosInterval::new(v,i))
            .collect();
        pvs.sort_by(|a,b| {
            let t = a.v[0].cmp(&b.v[0]);
            match t {
                Ordering::Equal => {
                    a.v[1].cmp(&b.v[1])
                },
                _ => t,
            }
        });
        let mut ans = vec![-1; n];
        for i in 0..n-1 {
            let t = pvs[i].v[1];
            match &pvs[i+1..].binary_search_by(|a| {
                a.v[0].cmp(&t)
            }) {
                Ok(idx) => {
                    ans[pvs[i].p] = pvs[i+1+idx].p as i32;
                },
                Err(idx) => {
                    if idx+1+i >= n {
                        ans[pvs[i].p] = -1;
                    } else {
                        ans[pvs[i].p] = pvs[i+1+idx].p as i32;
                    }
                }
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
    fn test_find_right_interval() {
        let test_cases = vec![
            (vec![vec![1,4], vec![2,3], vec![3,4]], vec![-1,2,-1]),
            (vec![vec![3,4],vec![2,3],vec![1,2]], vec![-1,0,1]),
            (vec![vec![1,2]], vec![-1]),
        ];
        for (nums, expect) in test_cases {
            assert_eq!(expect, Solution::find_right_interval(nums.clone()), "intervals: {:?}", nums);
        }
    }
}