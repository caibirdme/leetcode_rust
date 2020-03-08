use std::collections::HashMap;

impl Solution {
    pub fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
        if n == 1 {
            return 1f64;
        }
        let mut hash = HashMap::new();
        for edge in edges {
            let (x,y) = (edge[0], edge[1]);
            hash.entry(x).or_insert(vec![]).push(y);
            hash.entry(y).or_insert(vec![]).push(x);
        }
        let mut visited = vec![false; n as usize+1];
        visited[1] = true;
        let mut q = vec![(1, 0, 1f64)];
        let mut head = 0;
        while head < q.len() {
            let (x, step, prob) = q[head];
            head += 1;
            if let Some(ch) = hash.get(&x) {
                let number = ch.iter().filter(|&&id| !visited[id as usize]).count();
                if target == x {
                    if step == t || number == 0{
                        return prob;
                    }
                    return 0f64;
                }
                if step < t {
                    let next_prob = prob * 1f64/number as f64;
                    let next_step = step+1;
                    for &id in ch {
                        if !visited[id as usize] {
                            q.push((id, next_step, next_prob));
                            visited[id as usize] = true;
                        }
                    }
                }
            } else {
                if target == x {
                    return prob;
                }
            }
        }
        0f64
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_frog_position() {
        let test_cases = vec![
            (9, vec![vec![2,1],vec![3,1],vec![4,2],vec![5,3],vec![6,5],vec![7,4],vec![8,7],vec![9,7]],1,8, 0.0),
            (7, vec![vec![1,2],vec![1,3],vec![1,7],vec![2,4],vec![2,6],vec![3,5]], 20, 6, 0.16666666666666666),
            (7, vec![vec![1,2],vec![1,3],vec![1,7],vec![2,4],vec![2,6],vec![3,5]], 2, 4, 0.16666666666666666),
            (7, vec![vec![1,2],vec![1,3],vec![1,7],vec![2,4],vec![2,6],vec![3,5]], 2, 5, 0.33333333333),
            (7, vec![vec![1,2],vec![1,3],vec![1,7],vec![2,4],vec![2,6],vec![3,5]], 1, 7, 0.33333333333),
        ];
        for (n, edges, t, target, expect) in test_cases {
            let actual = Solution::frog_position(n, edges.clone(), t, target);
            assert!((actual-expect).abs() < 1e-5 ,"actual: {}, n:{}, edges: {:?}, t: {}, target: {}", actual, n, edges, t, target);
        }
    }
}