use std::collections::{BTreeSet, HashMap};

impl Solution {
    pub fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return 0;
        }
        let mut ans = 0;
        let mut tmp = vec![];
        Self::merge_sort(&mut nums, 0, n-1, &mut tmp, &mut ans);
        ans as i32
    }
    fn merge_sort(nums: &mut Vec<i32>, l: usize, r: usize, tmp: &mut Vec<i32>, ans: &mut usize) {
        if l >= r { return ; }
        let mid = (l+r)/2;
        Self::merge_sort(nums, l, mid, tmp, ans);
        Self::merge_sort(nums, mid+1, r, tmp, ans);
        let mut k = l;
        for i in mid+1..=r {
            if let Some(double) = nums[i].checked_mul(2) {
                while k <= mid && nums[k] <= double { k+=1;}
                *ans += mid+1-k;
            } else if nums[i] < 0 { // < i32::MIN overflow
                *ans += mid+1-k;
            } else { // > i32::MAX overflow
                break;
            }
        }
        let mut i = l;
        let mut j = mid+1;
        tmp.clear();
        while i <= mid && j <= r {
            if nums[i] < nums[j] {
                tmp.push(nums[i]);
                i+=1;
            } else {
                tmp.push(nums[j]);
                j+=1;
            }
        }
        while i <= mid {tmp.push(nums[i]); i+=1;}
        while j <= r {tmp.push(nums[j]); j+=1;}
        for i in 0..(r-l+1) {
            nums[l+i] = tmp[i];
        }
    }
    pub fn reverse_pairs_bit(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }
        let mut set = BTreeSet::new();
        for &v in &nums {
            set.insert(v as i64);
            set.insert(v as i64*2);
        }
        let n = set.len();
        let mut hash = HashMap::new();
        for (idx, &v) in set.iter().enumerate() {
            hash.insert(v, idx+1);
        }
        let mut ans = 0;
        let mut bits = vec![0; n+1];
        let mut total = 0;
        for &v in nums.iter() {
            ans += total - Self::query(&bits, *hash.get(&(v as i64*2)).unwrap());
            Self::update(&mut bits, *hash.get(&(v as i64)).unwrap());
            total += 1;
        }
        ans
    }
    fn update(bits: &mut Vec<i32>, idx: usize) {
        let mut idx = idx as i32;
        while idx < bits.len() as i32 {
            bits[idx as usize] += 1;
            idx += idx & -idx;
        }
    }
    fn query(bits: &Vec<i32>, idx: usize) -> i32 {
        let mut idx = idx as i32;
        let mut ans = 0;
        while idx > 0 {
            ans += bits[idx as usize];
            idx -= idx&-idx;
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_reverse_pairs() {
        let test_cases = vec![
            (vec![-100,-90,-80,-51,-50,1], 9),
            (vec![233,2000000001,234,2000000006,235,2000000003,236,2000000007,237,2000000002,2000000005,233,233,233,233,233,2000000004], 40),
        ];
        for (nums, expect) in test_cases {
            assert_eq!(Solution::reverse_pairs_bit(nums.clone()), Solution::reverse_pairs(nums.clone()),"nums: {:?}", nums);
        }
    }
}