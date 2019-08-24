impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        use std::cmp::min;
        let (r,g,b) = costs.into_iter().fold((0,0,0), |(red,green,blue),v| {
            let (r,g,b) = (v[0],v[1],v[2]);
            (min(green,blue)+r, min(red,blue)+g, min(red,green)+b)
        });
        min(r, min(g,b))
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_cost() {
        let test_cases = vec![
            (vec![vec![17,2,17],vec![16,16,5],vec![14,3,19]], 10),
        ];
        for (costs, expect) in test_cases {
            assert_eq!(Solution::min_cost(costs.clone()), expect, "costs: {:?}", costs);
        }
    }
}