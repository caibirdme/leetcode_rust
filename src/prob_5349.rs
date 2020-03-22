use std::collections::HashMap;

impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        if reserved_seats.is_empty() {
            return n*2;
        }
        let mut reserved = HashMap::new();
        for arr in reserved_seats {
            let (i, j) = (arr[0], arr[1]);
            *reserved.entry(i).or_insert(0) |= 1<<(j-1);
        }
        let mut ans = 0;
        for &v in reserved.values() {
            if v & 0b111111110 == 0 {
                ans += 2;
            } else if v & 0b11110 ==0 || v & 0b1111000 == 0 || v & 0b111100000 == 0 {
                ans += 1;
            }
        }
        ans + (n-reserved.len() as i32)*2
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_number_of_families() {
        let test_cases = vec![
            (5,
             vec![[4,7],[4,1],[3,1],[5,9],[4,4],[3,7],[1,3],[5,5],[1,6],[1,8],[3,9],[2,9],[1,4],[1,9],[1,10]],
            2,),
            (4, vec![[4,3],[1,4],[4,6],[1,7]], 4),

        ];
        for (n, seats, expect) in test_cases {
            assert_eq!(Solution::max_number_of_families(n, seats.iter().map(|v| vec![v[0], v[1]]).collect()), expect, "n: {}, seats: {:?}", n, seats);
        }
    }
}