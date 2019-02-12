use std::collections::BinaryHeap;
use std::cmp::Ordering;
struct Solution;
impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if buildings.len() == 0 {
            return vec![];
        }
        let mut edges = vec![];
        for building in buildings {
            let h = building[2];
            edges.push(Edge::new(building[0], h));
            edges.push(Edge::new(building[1], -h));
        }
        edges.sort();
        let mut res = vec![];
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        let mut remove: BinaryHeap<i32> = BinaryHeap::new();
        for edge in edges {
            if !edge.is_end() {
                if heap.is_empty() || edge.h > *heap.peek().unwrap() {
                    res.push(vec![edge.x, edge.h]);
                }
                heap.push(edge.h);
            } else {
                let h = -edge.h;
                remove.push(h);
                while !heap.is_empty() && !remove.is_empty() && *heap.peek().unwrap() == *remove.peek().unwrap() {
                    heap.pop();
                    remove.pop();
                }
                if heap.is_empty() {
                    res.push(vec![edge.x, 0]);
                } else if *heap.peek().unwrap() < -edge.h {
                    res.push(vec![edge.x, *heap.peek().unwrap()]);
                }
            }
        }
        res
    }
}

#[derive(Eq, PartialEq)]
struct Edge {
    x: i32,
    h: i32,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.cmp(&other.x).then_with(|| other.h.cmp(&self.h))
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Edge {
    fn new(x: i32, h: i32) -> Self {
        Self{x,h}
    }
    fn is_end(&self) -> bool {
        self.h < 0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_get_skyline() {
        assert_eq!(
            Solution::get_skyline(vec![vec![0,2,3], vec![2,5,3]]),
            vec![vec![0,3], vec![5,0]]
        );
        assert_eq!(
            Solution::get_skyline(vec![
            vec![2,9 ,10], vec![3 ,7 ,15], vec![5 ,12 ,12], vec![15 ,20 ,10], vec![19 ,24 ,8]
            ]),
            vec![
            vec![2 ,10], vec![3 ,15], vec![7, 12], vec![12, 0], vec![15, 10], vec![20, 8], vec![24, 0]
            ]
        );
    }
}