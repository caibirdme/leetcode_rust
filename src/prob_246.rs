use std::collections::HashMap;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}


impl Solution {
    pub fn is_strobogrammatic(num: String) -> bool {
        let map: HashMap<char,char> = hashmap!['0'=>'0', '1'=>'1', '6'=>'9', '8'=>'8', '9'=>'6'];
        let n = num.len();
        let num = num.as_bytes();
        for i in 0..=n/2 {
            if let Some(&c) = map.get(&(num[i] as char)) {
                if c != num[n-1-i] as char {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_is_strobogrammatic() {
        let test_cases = vec![
            ("69", true),
            ("6889", true),
            ("3", false),
            ("6688899", true),
        ];
        for (s, ok) in test_cases {
            assert_eq!(Solution::is_strobogrammatic(s.to_string()), ok, "s {}", s);
        }
    }
}