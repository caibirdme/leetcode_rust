
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        if gas.iter().sum::<i32>() < cost.iter().sum() {
            return -1;
        }
        let mut ans = -1;
        let mut rest = 0;
        let mut i = 0;
        while i < gas.len() {
            if ans == -1 && gas[i]<cost[i] {
                i+=1;
                continue;
            }
            if ans == -1 {
                rest = gas[i] - cost[i];
                ans = i as i32;
                i+=1;
            } else {
                rest += gas[i] - cost[i];
                if rest < 0 {
                    ans = -1;
                } else {
                    i+=1;
                }
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
    fn test_can_complete_circuit() {
        let test_cases = vec![
            (vec![1,2,3,4,5], vec![3,4,5,1,2], 3),
            (vec![2,3,4], vec![3,4,3], -1),
        ];
        for (gas, cost, expect) in test_cases {
            assert_eq!(Solution::can_complete_circuit(gas.clone(), cost.clone()), expect, "gas: {:?}, cost: {:?}", gas, cost);
        }
    }
}