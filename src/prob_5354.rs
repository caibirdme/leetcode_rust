use std::collections::HashMap;

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        if n < 2 {
            return n;
        }
        let mut hash = HashMap::new();
        for (idx, &parent) in manager.iter().enumerate() {
            hash.entry(parent as usize).or_insert(vec![]).push(idx);
        }
        let mut ans = 0;
        let mut q = vec![(head_id as usize, 0)];
        let mut head = 0;
        while head < q.len() {
            let (x, t) = q[head];
            head += 1;
            let nt = t + inform_time[x];
            if let Some(children) = hash.get(&x) {
                for &id in children {
                    q.push((id, nt));
                }
            } else {
                ans = ans.max(t);
            }
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

}