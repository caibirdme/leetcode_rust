use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Elem {
    idx: usize,
    vote: [i32; 26],
}

impl Ord for Elem {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in 0..26usize {
            if self.vote[i] != other.vote[i] {
                return self.vote[i].cmp(&other.vote[i]);
            }
        }
        other.idx.cmp(&self.idx)
    }
}

impl PartialOrd for Elem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn rank_teams(votes: Vec<String>) -> String {
        let n = votes.len();
        if n == 0 {
            return "".to_string();
        }
        if n == 1 {
            return votes[0].clone();
        }
        let m = votes[0].len();
        if m == 1 {
            return votes[0].clone();
        }
        let mut count = [[0; 26]; 26];
        for s in votes {
            for (i,&c) in s.as_bytes().iter().enumerate() {
                count[(c-b'A') as usize][i] += 1;
            }
        }
        let mut heap = BinaryHeap::new();
        for i in 0..26usize {
            for j in 0..26 {
                if count[i][j] > 0 {
                    heap.push(Elem{idx:i, vote: count[i].clone(),});
                    break;
                }
            }
        }
        let mut ans = vec![];
        while let Some(Elem{idx: idx, vote: _}) = heap.pop() {
            ans.push(idx as u8+b'A');
        }
        unsafe {std::str::from_utf8_unchecked(&ans).to_string()}
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_rank_teams() {
        let test_cases = vec![
            (vec!["ABC","ACB","BAC","BCA"], "ABC"),
            (vec!["M","M","M","M"],"M"),
            (vec!["ABC","ACB","ABC","ACB","ACB"], "ACB"),
            (vec!["WXYZ","XYZW"], "XWYZ"),
            (vec!["BCA","CAB","CBA","ABC","ACB","BAC"], "ABC"),
        ];
        for (votes, expect) in test_cases {
            assert_eq!(Solution::rank_teams(votes.iter().map(|v| v.to_string()).collect()), expect, "votes: {:?}", votes);
        }
    }
}