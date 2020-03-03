use std::collections::HashSet;

impl Solution {
    pub fn sequence_reconstruction(org: Vec<i32>, seqs: Vec<Vec<i32>>) -> bool {
        let n = org.len();
        let mut edges = vec![HashSet::new(); n+1];
        let mut count = vec![0; n+1];
        let mut emerged = vec![false; n+1];
        for seq in seqs {
            if seq.is_empty() {continue;}
            for i in 0..seq.len()-1 {
                let from = seq[i];
                let to = seq[i+1];
                if from > n as i32 || from <= 0 || to > n as i32 || to <= 0{
                    return false;
                }
                emerged[from as usize] = true;
                let before = edges[from as usize].len();
                edges[from as usize].insert(to as usize);
                if edges[from as usize].len() > before {
                    count[to as usize] += 1;
                }
            }
            let last = *seq.last().unwrap();
            if last <= 0 || last > n as i32 {
                return false;
            }
            emerged[last as usize] = true;
        }
        for i in 1..=n {
            if !emerged[i] {return false;}
        }
        let mut t = 0;
        let mut j = 0;
        for i in 1..=n {
            if count[i] == 0 {
                t += 1;
                j = i;
            }
        }
        for _ in 0..n {
            if t != 1 {
                return false;
            }
            count[j] = -1;
            t = 0;
            for &next in edges[j].iter() {
                count[next] -= 1;
                if count[next] == 0 {
                    j = next;
                    t+=1;
                }
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
    fn test_sequence_reconstruction() {
        let test_cases = vec![
            (
                vec![5,3,2,4,1],
                vec![vec![5,3,2,4],vec![4,1],vec![1], vec![3], vec![2,4], vec![1,10000000], vec![]],
                false,
            ),
            (
                vec![5,3,2,4,1],
                vec![vec![5,3,2,4],vec![4,1],vec![1], vec![3], vec![2,4], vec![10000000], vec![]],
                false,
            ),
            (
                vec![1],
                vec![vec![],vec![],vec![]],
                false,
            ),
            (
                vec![1,2,3],
                vec![
                    vec![1,2], vec![1,3],
                ],
                false,
            ),
            (
                vec![1,2,3],
                vec![
                    vec![1,2],
                ],
                false,
            ),
            (
                vec![1,2,3],
                vec![
                    vec![1,2], vec![1,3],vec![2,3],
                ],
                true,
            ),
            (
                vec![4,1,5,2,6,3],
                vec![
                    vec![5,2,6,3],vec![4,1,5,2],
                ],
                true,
            ),
        ];
        for (org,seqs, ok) in test_cases {
            assert_eq!(Solution::sequence_reconstruction(org.clone(), seqs.clone()), ok, "org: {:?}, seqs: {:?}", org, seqs);
        }
    }
}