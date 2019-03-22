impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let n = data.len();
        if n == 0 {
            return false;
        }
        let mut i = 0;
        while i < n {
            let m = Self::meta_bytes(data[i]);
            if m == 4 {
                return false;
            }
            if n-i < m {
                return false;
            }
            for _ in 0..m {
                i+=1;
                if !Self::is_80(data[i]) {
                    return false;
                }
            }
            i+=1;
        }
        true
    }
    #[inline]
    fn meta_bytes(n: i32) -> usize {
        if n&0xf8 == 0xf0 {
            3
        } else if n&0xf0 == 0xe0 {
            2
        } else if n&0xe0 == 0xc0 {
            1
        } else if n&0x80 == 0{
            0
        } else {
            4 // represent fail
        }
    }
    #[inline]
    fn is_80(n: i32) -> bool {
        return n&0xc0 == 0x80
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_valid_utf8() {
        let test_case = vec![
            (vec![240,162,138,147], true),
            (vec![197, 130, 1], true),
            (vec![235, 140, 4], false),
        ];
        for (data, expect) in test_case {
            assert_eq!(expect, Solution::valid_utf8(data.clone()), "data: {:?}", data);
        }
    }
}