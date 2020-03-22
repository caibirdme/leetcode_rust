impl Solution {
    pub fn find_the_distance_value(mut arr1: Vec<i32>, mut arr2: Vec<i32>, d: i32) -> i32 {
        if arr1.is_empty() || arr2.is_empty() {return 0;}
        arr1.sort();
        arr2.sort();
        let mut ans = 0;
        for &v in &arr1 {
            if let Err(pos) = arr2.binary_search(&(v-d)) {
                if pos == arr2.len() || Self::abs(v-arr2[pos]) > d {
                    if let Err(pos) = arr2.binary_search(&(v+d)) {
                        if pos == arr2.len() || Self::abs(v-arr2[pos]) > d {
                            ans += 1;
                        }
                    }
                }
            }
        }
        ans
    }
    fn abs(x: i32) -> i32 {
        if x < 0 {-x} else {x}
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_the_distance_value() {
        let test_cases = vec![
            (vec![4,5,8], vec![10,9,1,8], 2, 2),
            (vec![1,4,2,3], vec![-4,-3,6,10,20,30], 3, 2),
            (vec![2,1,100,3], vec![-5,-2,10,-3,7], 6, 1),
        ];
        for (arr1, arr2, d, expect) in test_cases {
            assert_eq!(Solution::find_the_distance_value(arr1.clone(), arr2.clone(), d), expect, "arr1: {:?}, arr2: {:?}, d:{}",arr1,arr2,d);
        }
    }
}