impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        let mut sum = vec![0i64; n+1];
        let mut acc = 0i64;
        for (i,v) in nums.into_iter().enumerate() {
            acc += v as i64;
            sum[i+1] = acc;
        }
        let mut cache: Vec<i64> = Vec::with_capacity(n);
        Self::merge_sort(&mut sum, &mut cache, 0, n, lower as i64, upper as i64) as i32
    }
    fn merge_sort(sum: &mut Vec<i64>, cache: &mut Vec<i64>, left: usize, right: usize, lower: i64, upper: i64) -> usize {
        if left == right {
            return 0;
        }
        let mid = (left+right) / 2;
        let mut res = Self::merge_sort(sum, cache, left, mid, lower, upper) + Self::merge_sort(sum,cache,mid+1, right, lower, upper);
        let (mut rk, mut rj) = (mid+1, mid+1);
        for i in left..=mid {
            while rk <= right && sum[rk]-sum[i]<=upper {rk+=1;}
            while rj <= right && sum[rj]-sum[i]<lower {rj+=1;}
            res += rk-rj;
        }
        let mut i = left;
        let mut j = mid+1;
        cache.clear();
        while i<=mid && j <= right {
            if sum[i]<sum[j] {
                cache.push(sum[i]);
                i+=1;
            } else {
                cache.push(sum[j]);
                j+=1;
            }
        }
        while i<=mid {
            cache.push(sum[i]);
            i+=1;
        }
        while j<=right {
            cache.push(sum[j]);
            j+=1;
        }
        for i in left..=right {
            sum[i] = cache[i-left];
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_count_range_sum() {
        let test_cases = vec![
            (vec![-2,5,-1], -2, 2, 3),
            (vec![1,], 0, 1, 1),
            (vec![1,2], 0, 3, 3),
        ];
        for (nums, lower, upper, expect) in test_cases {
            assert_eq!(
                Solution::count_range_sum(nums, lower, upper),
                expect
            );
        }
    }
}