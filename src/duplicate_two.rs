impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let length = nums.len();
        if length <= 1 || k <= 0{
            return false;
        }
        let mut arr = Vec::with_capacity(length);
        for (pos, &v) in nums.iter().enumerate() {
            arr.push((v,pos));
        }
        arr.sort_by(|a,b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
        let k = k as usize;
        let mut pre = *arr.first().unwrap();
        for (v,pos) in arr.into_iter().skip(1) {
            if v == pre.0 && pos-pre.1 <= k {
                return true;
            }
            pre = (v,pos);
        }
        false
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_contains_nearby_duplicate() {
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1,0,1,1], 1),
            true
        );
    }
}