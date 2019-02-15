impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.len() == 0 {
            return vec![];
        }
        let mut fsm = (*nums.first().unwrap(), *nums.first().unwrap());
        let mut res = vec![];
        for &i in nums.iter().skip(1) {
            if i == fsm.1 + 1 {
                fsm.1 = i;
            } else {
                res.push(Self::to_str(fsm));
                fsm = (i,i);
            }
        }
        res.push(Self::to_str(fsm));
        res
    }
    fn to_str(fsm: (i32,i32)) -> String {
        if fsm.0 == fsm.1 {
            fsm.0.to_string()
        } else {
            fsm.0.to_string() + "->" + fsm.1.to_string().as_str()
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_summary_ranges() {
        assert_eq!(
            Solution::summary_ranges(vec![0,1,2,4,5,7]),
            vec!["0->2".to_string(), "4->5".to_string(), "7".to_string()]
        );
        assert_eq!(
            Solution::summary_ranges(vec![0,2,3,4,6,8,9]),
            vec!["0".to_string(),"2->4".to_string(),"6".to_string(),"8->9".to_string()]
        );
        assert_eq!(
            Solution::summary_ranges(vec![0]),
            vec!["0".to_string()]
        );
    }
}