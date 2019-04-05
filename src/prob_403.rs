use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        if stones.len() <= 1 {
            return true;
        }
        if stones[1] != 1 {
            return false;
        }
        let target = *stones.last().unwrap() as i64;
        let mut stone = HashSet::new();
        for v in stones {
            stone.insert(v as i64);
        }
        let mut steps: HashMap<i64, HashSet<i64>> = HashMap::new();
        steps.entry(1).or_insert(HashSet::new()).insert(1);
        let mut queue = VecDeque::new();
        queue.push_back(1);
        while !queue.is_empty() {
            let cur = queue.pop_front().unwrap();

            for s in steps.remove(&cur).unwrap() {
                for i in -1..=1 {
                    let next = cur+s+i;
                    if next == target {
                        return true;
                    }
                    if next <= cur || next > target || !stone.contains(&next) { continue; }
                    if !steps.contains_key(&next) {
                        queue.push_back(next);
                    }
                    steps.entry(next).or_insert(HashSet::new()).insert(s+i);
                }
            }
        }
        false
    }
}

struct Solution;


#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_can_cross() {
        let test_cases = vec![
            (vec![0,1], true),
            (vec![0,1,3,5,6,8,12,17], true),
            (vec![0,1,2,3,4,8,9,11], false),
        ];
        for (stones, ok) in test_cases {
            assert_eq!(ok, Solution::can_cross(stones.clone()), "stones: {:?}", stones);
        }
    }
}