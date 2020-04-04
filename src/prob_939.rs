use std::collections::{BTreeMap, BTreeSet};
use std::iter::FromIterator;
use std::ops::Bound;

impl Solution {
    pub fn min_area_rect(points: Vec<Vec<i32>>) -> i32 {
        let mut vx = BTreeMap::new();
        let mut vy = BTreeMap::new();
        let mut ans = std::i32::MAX;
        for point in points {
            let (i,j) = (point[0], point[1]);
            vx.entry(i).or_insert(BTreeSet::new()).insert(j);
            vy.entry(j).or_insert(BTreeMap::new()).insert(i, ());
        }
        for (&x, y_points) in vx.iter() {
            if y_points.len() < 2 {
                continue;
            }
            let arr: Vec<i32> = Vec::from_iter(y_points.iter().cloned());
            let n = arr.len();
            for i in 0..n-1 {
                let y1 = arr[i];
                if vy.get(&y1).unwrap().len() < 2 {
                    continue;
                }
                for j in i+1..n {
                    let y2 = arr[j];
                    if vy.get(&y2).unwrap().len() < 2 {
                        continue;
                    }
                    for (&tx, _) in vy.get(&y2).unwrap().range((Bound::Excluded(&x), Bound::Unbounded)) {
                        if vx.get(&tx).unwrap().contains(&y1) {
                            ans = ans.min((y2-y1)*(tx-x));
                        }
                    }
                }
            }
        }
        if ans == std::i32::MAX {
            0
        } else {
            ans
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_area_rect() {
        let test_cases = vec![
            (vec![[1,1],[1,3],[3,1],[3,3],[4,1],[4,3]], 2),
            (vec![[1,1],[1,3],[3,1],[3,3],[2,2]], 4),
        ];
        for (points, expect) in test_cases {
            assert_eq!(Solution::min_area_rect(points.iter().map(|v| vec![v[0], v[1]]).collect()), expect, "points: {:?}", points);
        }
    }
}