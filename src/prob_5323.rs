impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_by(|&a,&b| {
            let na = Self::count(a);
            let nb = Self::count(b);
            if na == nb {
                a.cmp(&b)
            } else {
                na.cmp(&nb)
            }
        });
        arr
    }
    fn count(mut a: i32) -> i32 {
        let mut count = 0;
        while a > 0 {
            count += a&1;
            a >>= 1;
        }
        count
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
}