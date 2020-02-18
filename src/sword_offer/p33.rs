impl Solution {
    pub fn verify_postorder(postorder: Vec<i32>) -> bool {
        Self::verify(&postorder)
    }
    fn verify(order: &[i32]) -> bool {
        if order.is_empty() {
            return true;
        }
        let n = order.len();
        if n == 1 {
            return true;
        }
        let root = order[n-1];
        let mut lch = None;
        for i in (0..n-1).rev() {
            if order[i] < root {
                lch = Some(i);
                break;
            }
        }
        if let Some(idx) = lch {
            for i in 0..idx {
                if order[i] > root {
                    return false;
                }
            }
            if !Self::verify(&order[..idx]) {
                return false;
            }
        }
        let idx = if let Some(idx) = lch {idx+1} else {0};
        Self::verify(&order[idx..n-1])
    }
}

struct Solution;