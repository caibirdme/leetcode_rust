use std::collections::{BTreeMap, HashSet};

impl Solution {
    pub fn is_reflected(points: Vec<Vec<i32>>) -> bool {
        if points.len() <= 1 {
            return true;
        }
        let mut xv = BTreeMap::new();
        for point in points {
            let (x,y) = (point[0], point[1]);
            xv.entry(x).or_insert(HashSet::new()).insert(y);
        }
        let mut arr = vec![];
        for (k,v) in xv.into_iter() {
            let mut nums: Vec<i32> = v.into_iter().collect();
            nums.sort();
            arr.push((k, nums));
        }
        if arr.len() == 1 {
            return true;
        }
        let (mut l, mut r) = {
            let mid = arr.len() / 2;
            if arr.len() % 2 == 0 {
                (mid-1, mid)
            } else {
                (mid-1,mid+1)
            }
        };
        if l+1 != r && arr[l+1].0-arr[l].0 != arr[r].0-arr[l+1].0 {
            return false;
        }
        let mut mx = arr[l].0+arr[r].0;
        while r < arr.len() {
            if arr[l].0+arr[r].0 != mx || arr[l].1.len() != arr[r].1.len() {
                return false;
            }
            for (&a,&b) in arr[l].1.iter().zip(arr[r].1.iter()) {
                if a != b {
                    return false;
                }
            }
            r += 1;
            if r < arr.len() {
                l-=1;
            }
        }
        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_is_reflected() {
        let test_cases = vec![
            (vec![vec![0,0], vec![1,0], vec![2,0]], true),
            (vec![vec![0,0], vec![1,0], vec![3,0]], false),
            (vec![vec![-16,1], vec![16,1], vec![16,1]], true),
            (vec![vec![0,0], vec![0,0]], true),
        ];
        for (points, ok) in test_cases {
            assert_eq!(Solution::is_reflected(points.clone()), ok, "points: {:?}", points);
        }
    }
}