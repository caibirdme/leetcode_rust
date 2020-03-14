macro_rules! idx {
    ($c: expr) => {
        ($c-b'a') as usize
    };
}

impl Solution {
    pub fn remove_duplicate_letters_dfs(s: String) -> String {
        let mut count = [0; 26];
        for &c in s.as_bytes() {
            count[(c-b'a') as usize] += 1;
        }
        let to_delete = count.iter().fold(0i32, |a,b| if *b > 1 {a+*b-1} else {a} );
        if to_delete == 0 {
            return s;
        }
        let mut cur = String::new();
        let mut ans = String::new();
        let mut used = [false; 26];
        Self::remove(s.as_str(), &mut count, &mut used, to_delete, &mut cur, &mut ans);
        ans
    }
    fn remove(s: &str, count: &mut [i32; 26], used: &mut [bool;26], to_delete: i32, cur: &mut String, ans: &mut String) {
        if to_delete == 0 {
            if cur < ans || ans.is_empty() {
                let n = cur.len();
                cur.push_str(s);
                *ans = cur.clone();
                cur.truncate(n);
            }
            return;
        }
        if s.len() == 0 || to_delete as usize > s.len() {
            return;
        }
        if !ans.is_empty() && cur >= ans {
            return;
        }
        if to_delete as usize == s.len() {
            if cur < ans {
                *ans = cur.clone();
            }
            return;
        }
        let c = s.as_bytes()[0];
        let idx = (c-b'a') as usize;
        if count[idx] > 1 {
            //remove
            count[idx]-=1;
            Self::remove(&s[1..], count, used,to_delete-1, cur, ans);
            count[idx]+=1;
        }
        if !used[idx] {
            // remain
            used[idx] = true;
            cur.push(c as char);
            Self::remove(&s[1..], count, used, to_delete, cur, ans);
            cur.pop();
            used[idx] = false;
        }
    }

    pub fn remove_duplicate_letters(s: String) -> String {
        let mut count = [0; 26];
        let bs = s.as_bytes();
        for &c in bs {
            count[idx!(c)] += 1;
        }
        let mut stack = vec![bs[0]];
        let mut used = [false; 26];
        used[idx!(bs[0])] = true;
        count[idx!(bs[0])] -= 1;
        for i in 1..bs.len() {
            let c = bs[i];
            let pos = idx!(c);
            count[pos] -= 1;
            if used[pos] {continue;}
            while let Some(&top) = stack.last() {
                if c < top && count[idx!(top)] > 0 {
                    stack.pop();
                    used[idx!(top)] = false;
                } else {
                    break;
                }
            }
            stack.push(c);
            used[pos] = true;
        }
        unsafe {std::str::from_utf8_unchecked(&stack).to_string()}
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_remove_duplicate_letters() {
        assert_eq!(
            Solution::remove_duplicate_letters("bcabc".to_string()),
            "abc".to_string()
        );
        assert_eq!(
            Solution::remove_duplicate_letters("cbacdcbc".to_string()),
            "acdb".to_string()
        );
        assert_eq!(
            Solution::remove_duplicate_letters("dcba".to_string()),
            "dcba".to_string()
        );
        assert_eq!(
            Solution::remove_duplicate_letters("dadcba".to_string()),
            "adcb".to_string()
        );
        assert_eq!(
            Solution::remove_duplicate_letters("yioccqiorhtoslwlvfgzycahonecugtatbyphpuunwvaalcpndabyldkdtzfjlgwqk".to_string()),
            "ciorhsaebpunvdyktzfjlgwq".to_string()
        );
        assert_eq!(
            Solution::remove_duplicate_letters("yiklorymxepctlnomfmymitulgfuudxturmemjxxlloevwyfriazwyckgbfogfrppnsomjfhoobirytzzksemgrcbcegbbhaurrrlyxquuoivdcykcpnntgrktwtmgstjrvsvajfukhxwgvsvgzwoatnnzszksxstzkojmyuriyriyqkaqghoxilykyxepnsjeybgxxwyyornzxzttsylsoqlumzwlsdxvzgjfpwwoejsieeyoremvqfyekmxdsabogijmqxdruiydlkrvobwqmlmahmfpwbopbdxhinowqavdasnkeagpjvznzfmlllydgosztljnkrkpjhsqtjxjumzasfitacjqenwcskkkifgzatcevfwererjjabmmmdsnuacxzrgjyytbmxccagjbemkmemjpaqwpjdsunvmfuromfhmumhlzycbhptfjuodlgjxuxcggtotaxjlqbccghyplvtgrwwlhmriwnecdhjmbpzdaqgpyhinawvmxjyiptiroxtuwybcjjkqcirscdqbakpwdiabgirknpvlwmvspufpdqchvbqbspyznfuscidqcbtcvwsqgjjdfpnuhgpxkgikvagtbhnssycxpefsqxbcgtubdmtcojbzpcjvfoslunoiixxdakfczg".to_string()),
            "abcdefghijklmnoprqstvwuxyz".to_string()
        );
    }
}