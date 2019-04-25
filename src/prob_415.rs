impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        if num1.len() < num2.len() {
            return Self::add_strings(num2, num1);
        }
        if num1 == "0" {
            return num2;
        } else if num2 == "0" {
            return num1;
        }
        // 保证n>=m
        let n = {
            if num1.len() % 8 == 0 {
                num1.len()/8
            } else {
                num1.len()/8 +1
            }
        };
        let mut s1 = vec![0; n+1];
        let num1 = num1.as_bytes();
        let mut i = num1.len()-1;
        let mut pos = 0;
        while i>=0 {
            let mut total = 0;
            let mut cur = 1;
            let mut rest = 8;
            while i > 0 && rest > 0 {
                total += (num1[i]-b'0') as i32 * cur;
                cur *= 10;
                rest -= 1;
                i-=1;
            }
            if i == 0 && rest > 0 {
                total += (num1[0]-b'0') as i32 *cur;
                s1[pos] = total;
                pos+=1;
                break;
            }
            s1[pos] = total;
            pos+=1;
        }
        let num2 = num2.as_bytes();
        i = num2.len()-1;
        pos = 0;
        let one_hundred_million = 100000000;
        while i>=0 {
            let mut total = 0;
            let mut cur = 1;
            let mut rest = 8;
            while i > 0 && rest > 0 {
                total += (num2[i]-b'0') as i32 * cur;
                cur *= 10;
                rest -= 1;
                i-=1;
            }
            if i == 0 && rest > 0 {
                total += (num2[0]-b'0') as i32 * cur;
                s1[pos] += total;
                if s1[pos] >= one_hundred_million {
                    s1[pos+1] += 1;
                    s1[pos] -= one_hundred_million;
                }
                pos+=1;
                break;
            }
            s1[pos] += total;
            if s1[pos] >= one_hundred_million {
                s1[pos+1] += 1;
                s1[pos] -= one_hundred_million;
            }
            pos+=1;
        }
        while pos < n && s1[pos]>=one_hundred_million {
            s1[pos] -= one_hundred_million;
            s1[pos+1] += 1;
            pos+=1;
        }
        i = n;
        while i > 0 && s1[i] == 0 {
            i-=1;
        }
        let mut ans = s1[i].to_string();
        for j in (0..i).rev() {
            ans += format!("{:08}", s1[j]).as_str();
        }
        return ans;
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_add_strings() {
        let test_cases = vec![
            ("1", "9999999999999999999999999", "10000000000000000000000000"),
            ("9999999999999999999", "1", "10000000000000000000"),
            ("19999999999999999", "2", "20000000000000001"),
            ("123456789", "987654321", "1111111110"),
            ("199999999", "2", "200000001"),
            ("99999999", "1", "100000000"),
            ("0", "100", "100"),
            ("100", "0", "100"),
            ("0", "0", "0"),
        ];
        for (s1,s2,expect) in test_cases {
            assert_eq!(expect, Solution::add_strings(s1.clone().to_string(), s2.clone().to_string()), "s1: {}, s2: {}", s1, s2);
        }
    }
}