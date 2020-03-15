use std::collections::BTreeMap;

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut arr = vec![];
        for build in buildings {
            let (l, r, h) = (build[0], build[1], build[2]);
            arr.push((l, h));
            arr.push((r, -h));
        }
        arr.sort_by(|a,b| {
            if a.0 == b.0 {
                return b.1.cmp(&a.1);
            }
            a.0.cmp(&b.0)
        });
        let mut tree = BTreeMap::new();
        let mut x = 0;
        let mut height = 0;
        let mut ans = vec![];
        for (pos, h) in arr {
            if h > 0 {
                *tree.entry(h).or_insert(0) += 1;
            } else {
                let abs_h = -h;
                *tree.get_mut(&abs_h).unwrap() -= 1;
                if *tree.get(&abs_h).unwrap() == 0 {
                    tree.remove(&abs_h);
                }
            }
            let max_h = *tree.keys().last().unwrap_or(&0);
            if max_h != height {
                x = pos;
                height = max_h;
                ans.push(vec![x, height]);
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
    fn test_get_skyline() {
        let test_cases = vec![
            (vec![
                vec![2,9,10],vec![3,7,15],vec![5,12,12],vec![15,20,10],vec![19,24,8],
            ], vec![
                vec![2,10], vec![3,15],vec![7,12],vec![12,0],vec![15,10],vec![20,8],vec![24,0],
            ]),
        ];
        for (buildings, expect) in test_cases {
            assert_eq!(Solution::get_skyline(buildings.clone()), expect, "buildings: {:?}", buildings);
        }
    }
}