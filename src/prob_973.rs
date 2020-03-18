impl Solution {
    pub fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let n = points.len();
        if n == k as usize {
            return points;
        }
        Self::sort(&mut points, 0, n-1, k as usize);
        Vec::from(&points[..k as usize])
    }
    fn sort(data: &mut Vec<Vec<i32>>, l: usize, r: usize, k: usize) {
        if r-l+1 == k {
            return;
        }
        let m = (l+r)>>1;
        let piv = Self::dist(unsafe{data.get_unchecked(m)});
        data.swap(l, m);
        let mut i = l;
        let mut j = r;
        while i < j {
            while j > i && Self::dist(unsafe{data.get_unchecked(j)}).gt(&piv) {j-=1;}
            while i < j && Self::dist(unsafe{data.get_unchecked(i)}).le(&piv) {i+=1;}
            if i != j {
                data.swap(i,j);
                j -= 1;
            }
        }
        data.swap(l, j);
        let t = j-l+1;
        if k < t {
            Self::sort(data, l, j-1, k);
        } else if k > t {
            Self::sort(data, j+1, r, k-t);
        }
    }
    fn dist(v: &Vec<i32>) -> i32 {
        unsafe {
            let &p = v.get_unchecked(0);
            let &q = v.get_unchecked(1);
            p*p+q*q
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_k_closest() {
        let test_cases = vec![
            (vec![vec![3,3], vec![-2,4], vec![5,-1]], 2, vec![vec![3,3], vec![-2,4]]),
            (vec![vec![1,3], vec![-2,2]], 1, vec![vec![-2,2]]),
        ];
        for (points, k, expect) in test_cases {
            assert_eq!(Solution::k_closest(points.clone(), k), expect, "points: {:?}, k:{}", points, k);
        }
    }
}