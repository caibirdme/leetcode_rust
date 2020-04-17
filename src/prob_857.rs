use std::collections::BinaryHeap;

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let mut arr: Vec<(i32, f64)> = quality.iter().zip(wage.iter()).map(|(&q, &w)| (q, w as f64/q as f64)).collect();
        arr.sort_by(|x,y| {
            x.1.partial_cmp(&y.1).unwrap()
        });
        let mut total = 0;
        let mut h = BinaryHeap::new();
        for i in 0..k as usize {
            total += arr[i].0;
            h.push(arr[i].0);
        }
        let mut ans = total as f64*arr[k as usize-1].1;
        let n = quality.len();
        for i in k as usize..n {
            let top = *h.peek().unwrap();
            if top <= arr[i].0 {continue;}
            h.pop();
            h.push(arr[i].0);
            total = total - top + arr[i].0;
            ans = ans.min(arr[i].1 * total as f64);
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_mincost_to_hire_workers() {
        let test_cases = vec![
            (vec![3,1,10,10,1], vec![4,8,2,2,7], 3, 30.66667),
            (vec![10,20,5], vec![70,50,30], 2, 105.00),
        ];
        for (q, w, k, expect) in test_cases {
            let actual = Solution::mincost_to_hire_workers(q.clone(), w.clone(), k);
            assert_eq!((actual-expect).abs() < 1e-5,true, "q: {:?}, w: {:?}, k:{}, actual: {}, expect: {}",q,w,k,actual,expect);
        }
    }
}