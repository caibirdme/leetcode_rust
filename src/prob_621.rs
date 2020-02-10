use std::collections::BinaryHeap;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut count = vec![0; 26];
        for &c in &tasks {
            count[(c as u8-b'A') as usize] += 1;
        }
        let mut heap: BinaryHeap<i32> = count.into_iter().filter(|&v| v > 0).collect();
        let mut ans = 0;
        let mut q = vec![];
        let nn = n+1;
        while !heap.is_empty() {
            let mut t = 0;
            q.clear();
            while !heap.is_empty() && t <= n {
                let rest = heap.pop().unwrap()-1;
                if rest > 0 {
                    q.push(rest);
                }
                t+=1;
            }
            if q.is_empty() {
                ans += t;
            } else {
                ans += nn;
                for &v in &q {
                    heap.push(v);
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
    fn test_least_interval() {
        let test_cases = vec![
            (vec!['A','A','A','B','B','B','C','C'],2,8),
            (vec!['A','A','A','B','B','B','C','C','C'],2,9),
            (vec!['A','A','A','B','B','B','C','C','Z'],2,9),
            (vec!['A','A','A','B','B','B','C','C','Z','Z'],2,10),
            (vec!['A','A','A','B','B','B'], 2, 8),
        ];
        for (task, n, expect) in test_cases {
            assert_eq!(Solution::least_interval(task.clone(), n), expect, "task:{:?}, n:{}",task,n);
        }
    }
}