use std::collections::{HashSet, HashMap};
use std::cmp::max;

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = start_time.len();
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return profit[0];
        }
        let mut tuple = vec![];
        let mut pos = HashSet::new();
        for i in 0..n {
            tuple.push((start_time[i], end_time[i], profit[i]));
            pos.insert(start_time[i]);
            pos.insert(end_time[i]);
        }
        let mut dict = HashMap::new();
        let mut pos_arr: Vec<i32> = pos.iter().map(|v| *v).collect();
        pos_arr.sort();
        let m = pos_arr.len();
        let mut j = 0;
        let mut count = 0;
        let mut pre = pos_arr[0]-1;
        while j < m {
            dict.insert(pos_arr[j], count);
            pre = pos_arr[j];
            j+=1;
            count+=1;
            while j < m && pos_arr[j] == pre {j+1;}
        }
        tuple.sort_by(|a,b| a.1.cmp(&b.1));
        let mut plans: Vec<Vec<(usize, i32)>> = vec![vec![]; count];
        for (s,e,v) in tuple {
            plans[*dict.get(&e).unwrap()].push((*dict.get(&s).unwrap(), v));
        }
        let mut f = vec![0; count];
        for i in 1..count {
            let mut t = f[i-1];
            for (s,v) in plans[i].iter() {
                t = max(t, f[*s]+*v);
            }
            f[i] = t;
        }
        f[count-1]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_job_scheduling() {
        let test_cases = vec![
            (
                vec![1],
                vec![5],
                vec![3],
                3,
            ),
            (
                vec![1,2,3,3],
                vec![3,4,5,6],
                vec![50,10,40,70],
                120,
            ),
            (
                vec![1,2,3,4,6],
                vec![3,5,10,6,9],
                vec![20,20,100,70,60],
                150,
            ),
            (
                vec![1,1,1],
                vec![2,3,4],
                vec![5,6,4],
                6,
            ),
        ];
        for (s,e,v, expect) in test_cases {
            assert_eq!(Solution::job_scheduling(s.clone(), e.clone(), v.clone()), expect, "start: {:?}, end: {:?}, value: {:?}", s,e,v);
        }
    }
}