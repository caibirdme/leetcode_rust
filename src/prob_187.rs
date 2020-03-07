use std::collections::HashMap;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        if s.len() < 10 {
            return vec![];
        }
        let bs = s.as_bytes();
        let mut hash = HashMap::new();
        let mut i = 0;
        let mut t = 0;
        while i < 10 {
            t |= Self::get_hash_key(bs[i]) << (2*i);
            i+=1;
        }
        hash.insert(t, 1);
        while i < bs.len() {
            t >>= 2;
            t |= Self::get_hash_key(bs[i]) << 18;
            *hash.entry(t).or_insert(0) += 1;
            i+=1;
        }
        hash.into_iter().filter(|(_,v)| *v > 1).map(|(k,_)| {
            let mut s = vec![];
            for i in 0..10 {
                let w = (k >> (2*i)) & 3;
                s.push(match w {
                    0 => b'A',
                    1 => b'C',
                    2 => b'G',
                    3 => b'T',
                    _ => unreachable!(),
                });
            }
            unsafe {std::str::from_utf8_unchecked(&s).to_string()}
        }).collect()
    }
    fn get_hash_key(x: u8) -> i32 {
        match x {
            b'A' => 0,
            b'C' => 1,
            b'G' => 2,
            b'T' => 3,
            _ => unreachable!(),
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_repeated_dna_sequences() {
        let test_cases = vec![
            (vec!["AAAAAAAAAAAA"], vec!["AAAAAAAAAA"]),
            //(vec!["AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"],vec!["AAAAACCCCC", "CCCCCAAAAA"]),
        ];
        for (s, expect) in test_cases {
            assert_eq!(Solution::find_repeated_dna_sequences(s.iter().map(|v| v.to_string()).collect()), expect, "s: {:?}", s);
        }
    }
}