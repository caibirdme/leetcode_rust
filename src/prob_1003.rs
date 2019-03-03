impl Solution {
    pub fn is_valid(s: String) -> bool {
        let n = s.len();
        if n == 0 || n%3 != 0 {
            return false;
        }
        let (mut a,mut b) = (0,0);
        for c in s.chars() {
            match c {
                'a' => {a+=1;},
                'b' => {
                    b+=1;
                    if b > a {
                        return false;
                    }
                },
                'c' => {
                    a-=1;
                    b-=1;
                    if b < 0 || a < 0 {
                        return false;
                    }
                },
                _ => {unreachable!();},
            }
        }
        a==0&&b==0
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_is_valid() {
        let test_cases = vec![
            ("aabcbabcc", true),
            ("abc", true),
            ("aabaabcbccbc", true),
            ("aabcbc", true),
            ("abcabcababcc", true),
            ("abccba", false),
            ("cababc", false),
            ("bc", false),
            ("aaaccbcbabcacbaaccbaabbababaaaaacabcaaacabbaccaabccaccababbabbaaaaabccacacabaabcabcbcbacbcbccabcbaacbacbbbabbcaacbcaabcbabcacaabbbacacaabacbbacbbcacbcabcacbbaccbcbcbcaabbcccacbccbcaacbccccabbcbccbcccbccccbcbacabbccabbabbcccaaabacbcaaaacaaaabbbabcbcbcaabcaccbcabaaaccbbccacabbacaaabacacaabbabbccccababbcbcccbccccabacbccbcbcabcbacbbabcaacbbccaabbbacaacbbcaaaabbbbccbcbbbccbbcbabbcacbbbcbaccbccbaacccabbbaabccbaccbbacaacbabbaccbabbabcccaaacacaaccbbbbcccbcabbabbbababbbabbcabaacbbcbaccbacbaacbccccbcaccacacabaccccbccbbbaaabcbccaccaccbcacbcacbaabccaabccbaacabccbbbacbbcaaabbccbcbaccacababbcaacabccccbccbacbbaccaaaccacbacaaaaccbbcacbcccbacbbbbacabbcbbaacbaabaaabbbabcababccbbbbbaaccaabaaabccbccbaacbaaaaaabcbacccbbccacccccbbbcacbbacaaaccbcacbcbcbcbcbaaccabbcccbababcccbabcbbbaabcababcbaabbabbbaacbbaababbbaccbaaabbbaaabbabbcbbccabcccccabbcbaaacbcacacccbabbaaabacbbabbaacabaabbcbbcbbcbbaacbccacaacaccbbcacacbccbabbabbcbccaabccbacbbcacbbbaaaaccabbacccaaacbacabaccbbcaabcbcabcbcbbacccabbabcacbcbaaaacbbcbbccabbaacbcaabccacbccbacbccaacbacbcababcaacacccaacbcabbaabcccaaacccbbacabaaccbaacbbcbabcccbacbbbcaabcbbbbbaaabcbccabcccabaaaacaabaacaacaabacbcccccaaabaaabcaacbcacccccabbbacccabcabbaccabbaccacbacabbacaacacaabbacaccaabcbaccacbbbabcacbbcbccabcabbccaccabbbabcbabbaaacccbaacbaabcbcaccbbcbacbcaaabbccacacbcbcbcacabaaacaccbacbcaacacacccbccabbbbacabacaacacacaccabaacaccbbbcbbacacbaacaaaacacbaabbaacbaacaaabcbbbaaccabbabaccabcaccabccbbacccbcacabacbacaacacbbcbbbcbaaaacccacbacabbaaabaaccccacccacaacbaccaccbccabbbbccacccacccbbacbcbaccbcaacbacbbbbabaacabaaccabbabccbacaabbcacbaacaacabcbbbbbcbaacaaccbbaababbbcbcabaccbccaabccaccaacacbbcbbbcabbcbacabaccbccabcbbabcacbacaaccccaaccacabbccaccccbcacbaacbabcaccbbabcacabbbbbcccabbabcabbbaacbbccbcabacabbcbcaaabbcabbcbacaabcacaccacacbcbcbbcbaaaacaacbcbbaccacccbbaacbbaabccaabbacabcbbacbbbbbbabababacbbcbbabcccabbbaaabcababaabbaacbbacccccccccbbcaaaabcbacaaaccaacbbacbcccacbbbcbacaacacbababcacccbabccaccacccabbcbccbcbccbaccbbbbbbcabacccccbbbcabcbacbacacabbcbbbccbbaaaabbbbcbaaaccbabbcbabaaaacbacbbccbbbbaacbbbbabcacabaaaccbcccabacaacbcaccabababaaabbbbbababbaabcaababaacccbbaabcbabbbccbaccabaaaaacabaababbcbbcaaaaccbababbbbbacbaccabaccaccacaacabaabacabaabbabacaacbccbcbaacbaaaabaabbcacaabbcbcaabaccbbacbaccabbcacabcbbbbacabcaacaabcbbacbabbbbaabcaacaababaabcbbcacbbabccbbaababaccabcbbbcaaccbbbbbccbacbacccbbaacacbcaccbcbbacbacaccaccaabbcaabacbcbbababbbcbccbacbbbacbabcabacacbacaacaaccababbcacaaabbbaacbcbcbcbbaabaaabbbbcaaaaacbcaacbcaacbcbbaccacccbaacbacbbabcabcbcccabbbbccabcbbbcabaaccacaacacabbcaaaccbbbbccccbaaabbbaccbbccacaabcbbbccaccabaccccaabacacacabacaabcbaaccaacbbbabaaaccacaaabaccccbcbcacabaaabccabaabbbcbcbaaaacbcbbbbbbbbbccbbcbbaaabbcbababbbcbbabacbaacbbcbacaabcbaabacbaabacbbcaaccbabaabbcbbaacbaccbccbbccbcbabcccaaacbbcaacabccbcbaacabacacaaaccbacbbcacacabbacbbaccbacaaacabcabbbaaabbbbcbcabcbcacbbabaabbbccacbacaccbcccbacccacbbcababaaabaaaabbabbcbbbbbcbbaaacccbabcacaaacacbaacabaccacabacaacbccaabcbcbbaccaacabbabbcabcabababbcbccbaccbbbaaccacbaaacbcabcbbcbbcbbcacaacaaabaaaabcbcacbaccbacaccbbcbcbabbccbbacacbcaccaabcbcaacabaccaaacbccbaaabccaacccbabccabccabbccacbbcbcbcccaacaccbbcabbbcabcbbcccabcbabcacabaabccaacbcccaabacaacbbbcabcccaabbbacccbcacccbcbabbcccaabccabccbabcbcbaabbcaabcbcaabbbcbbabcaccacbababbabacaaaabbccabcbccababbabbbbbcbcbbabaccabababbacbcababaccbbbaaacbbccabbcccccabaaaccbbbabbccbbbcbabcaaaabbbabbbacacbcabaaacabccbbcaaacccbcbbaaabbbacaaccacabacacbbacccaaabbcbaabbbabaacacbbaaabbaaaacbabaacbacbbccaacaacababbbcbbbbabbcabaababbaaabaaaccbacbaacccccbbbbccbbacaaabbbccbaabccabcabaccaacbcaaccabcccacbbcaccbcaacaccaababccbabccbabccbacbacacbccababcbbaabacabacccaacbacbcccacbaccababcccaaaaa", false),
        ];
        for (s, ok) in test_cases {
            let st = s.to_string();
            assert_eq!(
                Solution::is_valid(st.clone()),
                ok,
                "st:{}, len:{}", st, st.len()
            );
        }
    }
}