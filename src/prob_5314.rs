use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        if n==1{
            return 0;
        }
        if arr[0] == arr[n-1] {
            return 1;
        }
        let mut new_arr = vec![arr[0]];
        let mut pre = arr[0];
        let mut double = true;
        for i in 1..n {
            if arr[i] != pre {
                new_arr.push(arr[i]);
                pre = arr[i];
                double = true;
            } else if double {
                new_arr.push(arr[i]);
                double = false;
            }
        }
        let mut hash = HashMap::new();
        for (i, &v) in new_arr.iter().enumerate() {
            hash.entry(v).or_insert(vec![]).push(i);
        }
        let n = new_arr.len();
        let mut used = vec![false; n];
        used[0] = true;
        let mut set = HashSet::new();
        let mut q = vec![(0usize,0i32)];
        let mut head = 0;
        let target = n-1;
        while head < q.len() {
            let (pos, step) = q[head];
            head+=1;
            let next_step = step+1;
            if pos > 0 && !used[pos-1] {
                used[pos-1] = true;
                q.push((pos-1, next_step));
            }
            if pos+1<n && !used[pos+1]{
                if pos+1 == target {
                    return next_step;
                }
                used[pos+1]= true;
                q.push((pos+1, next_step));
            }
            if !set.contains(&new_arr[pos]) {
                if let Some(s) = hash.get(&new_arr[pos]) {
                    for (_,&next_pos) in s.iter().enumerate() {
                        if !used[next_pos] {
                            used[next_pos] = true;
                            q.push((next_pos, next_step));
                            if next_pos == target {
                                return next_step;
                            }
                        }
                    }
                }
                set.insert(new_arr[pos]);
            }
        }
        (n-1) as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_jumps() {
        let mut arr = vec![7; 50000];
        arr.push(11);
        let test_cases = vec![
            (vec![11,22,7,7,7,7,7,7,7,22,13], 3),
            (arr, 2),
            (vec![1,2,3,4], 3),
            (vec![100,-23,-23,404,100,23,23,23,3,404], 3),
            (vec![7],0),
            (vec![7,6,9,6,9,6,9,7], 1),
            (vec![6,1,9], 2),
        ];
        for (arr, expect) in test_cases {
            assert_eq!(Solution::min_jumps(arr.clone()), expect, "arr: {:?}", arr);
        }
    }
}