use std::cmp::Ordering;
impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort_by(|a,b| {
            if a > b {
                Ordering::Less
            } else if a==b {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        });
        let n = citations.len();
        let mut ans = 0;
        for (mut i,&v) in citations.iter().enumerate() {
            i+=1;
            if v as usize >= i {
                ans = i;
            }
        }
        ans as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_h_index() {
        assert_eq!(
            Solution::h_index(vec![3,0,6,1,5]),
            3
        );
        assert_eq!(
            Solution::h_index(vec![4,0,6,1,5]),
            3
        );
    }
}