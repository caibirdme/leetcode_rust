impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = people.len();
        if n <= 1 {
            return people;
        }
        people.sort_by(|a,b| {
            if a[0] == b[0] {
                a[1].cmp(&b[1])
            } else {
                b[0].cmp(&a[0])
            }
        });
        let mut ans = vec![];
        for item in people {
            ans.insert(item[1] as usize, item);
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_reconstruct_queue() {
        let test_cases = vec![
            (vec![vec![7,0], vec![4,4], vec![7,1], vec![5,0], vec![6,1], vec![5,2]], vec![vec![5,0], vec![7,0], vec![5,2], vec![6,1], vec![4,4], vec![7,1]]),
            (vec![vec![1,1], vec![2,0]], vec![vec![2,0], vec![1,1]]),
        ];
        for (num, expect) in test_cases {
            assert_eq!(Solution::reconstruct_queue(num.clone()), expect, "num: {:?}", num);
        }
    }
}