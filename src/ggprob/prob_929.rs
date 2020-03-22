use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        use std::str::from_utf8_unchecked;
        let mut set = HashMap::new();
        for email in emails {
            let email = email.as_bytes();
            let mut k = 0;
            while email[k] != b'@' {k+=1;}
            let mut local_name = vec![];
            for i in 0..k {
                if email[i] == b'+' { break; }
                if email[i] != b'.' {local_name.push(email[i]);}
            }
            set.entry(unsafe{from_utf8_unchecked(&email[k+1..]).to_string()}).or_insert(HashSet::new()).insert(unsafe{from_utf8_unchecked(&local_name).to_string()});
        }
        set.values().fold(0, |pre,v| pre+v.len()) as i32
    }
}

struct Solution;