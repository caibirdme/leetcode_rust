impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }
        let unit = ["", "Thousand", "Million", "Billion"];
        let ss = num.to_string().chars().rev().collect::<String>();
        let mut s = ss.as_bytes();
        let mut uidx = 0;
        let mut ans = vec![];
        while s.len() >= 3 {
            let cur = Self::convert(&s[..3]);
            if cur != "Zero" {
                let u = unit[uidx];
                ans.push(Self::concat(cur, u));
            }
            s = &s[3..];
            uidx+=1;
        }
        match s.len() {
            1 => {
                ans.push(
                    Self::concat(Self::convert_1(s[0]).to_string(), unit[uidx])
                );
            },
            2 => {ans.push(Self::concat(Self::convert_2(s), unit[uidx]));},
            _ => {},
        }
        if ans.len() == 1 {
            ans[0].clone()
        } else {
            ans.iter().rev().fold(String::new(), |acc,b| {
                if acc.is_empty() {
                    b.to_string()
                } else {
                    acc + " " + b.as_str()
                }
            })
        }
    }

    fn concat(s: String, u: &str) -> String {
        if u.is_empty() {
            s
        } else {
            s + " " + u
        }
    }

    fn convert(s: &[u8]) -> String {
        let h = s[2];
        match h {
            b'0' => Self::convert_2(&s[..2]),
            _ => {
                let t = Self::convert_1(h).to_string() + " Hundred";
                let res =  Self::convert_2(&s[..2]);
                if res == "Zero" {
                    t
                } else {
                    t+ " " +res.as_str()
                }
            },
        }
    }

    fn convert_2(s: &[u8]) -> String {
        const DICT: [&str; 10] = ["","","Twenty","Thirty","Forty","Fifty","Sixty","Seventy","Eighty","Ninety"];
        let (h,l) = (s[1], s[0]);
        match h {
            b'0' => Self::convert_1(l).to_string(),
            b'1' => {
                match l {
                    b'0' => "Ten".to_string(),
                    b'1' => "Eleven".to_string(),
                    b'2' => "Twelve".to_string(),
                    b'3' => "Thirteen".to_string(),
                    b'4' => "Fourteen".to_string(),
                    b'5' => "Fifteen".to_string(),
                    b'6' => "Sixteen".to_string(),
                    b'7' => "Seventeen".to_string(),
                    b'8' => "Eighteen".to_string(),
                    b'9' => "Nineteen".to_string(),
                    _ => unreachable!(),
                }
            },
            _ => {
                let t = DICT[(h-b'0') as usize].to_string();
                match l {
                    b'0' => t,
                    _ => {
                        t + " "+ Self::convert_1(l)
                    },
                }
            },
        }
    }

    fn convert_1(s: u8) -> &'static str {
        match s {
            b'0' => "Zero",
            b'1' => "One",
            b'2' => "Two",
            b'3' => "Three",
            b'4' => "Four",
            b'5' => "Five",
            b'6' => "Six",
            b'7' => "Seven",
            b'8' => "Eight",
            b'9' => "Nine",
            _ => unreachable!(),
        }
    }

}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_number_to_words() {
        assert_eq!(
            Solution::number_to_words(1000010),
            "One Million Ten"
        );
        assert_eq!(
            Solution::number_to_words(5),
            "Five"
        );
        assert_eq!(
            Solution::number_to_words(123017),
            "One Hundred Twenty Three Thousand Seventeen"
        );
        assert_eq!(
            Solution::number_to_words(123000),
            "One Hundred Twenty Three Thousand"
        );
        assert_eq!(
            Solution::number_to_words(2000001),
            "Two Million One"
        );
        assert_eq!(
            Solution::number_to_words(100001),
            "One Hundred Thousand One"
        );
        assert_eq!(
            Solution::number_to_words(100000),
            "One Hundred Thousand"
        );
        assert_eq!(
            Solution::number_to_words(1000),
            "One Thousand"
        );
        assert_eq!(
            Solution::number_to_words(112),
            "One Hundred Twelve"
        );
        assert_eq!(
            Solution::number_to_words(11),
            "Eleven"
        );
        assert_eq!(
            Solution::number_to_words(0),
            "Zero"
        );
        assert_eq!(
            Solution::number_to_words(10),
            "Ten"
        );
        assert_eq!(
            Solution::number_to_words(12345),
            "Twelve Thousand Three Hundred Forty Five"
        );
        assert_eq!(
            Solution::number_to_words(1234567),
            "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
        );
        assert_eq!(
            Solution::number_to_words(1234567891),
            "One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One"
        );
    }
}