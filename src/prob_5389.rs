use std::collections::{HashMap, BTreeSet};

impl Solution {
    pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut hash = HashMap::new();
        let mut numbers = BTreeSet::new();
        for s in &orders {
            let num = s[1].parse::<i32>().unwrap();
            let item = s[2].clone();
            numbers.insert(num);
            *hash.entry(item).or_insert(HashMap::new()).entry(num).or_insert(0) += 1;
        }
        let mut menu: Vec<String> = hash.keys().map(|v| v.clone()).collect();
        menu.sort();
        let mut ans = vec![];
        let mut title = vec!["Table".to_owned()];
        for s in &menu {
            title.push(s.clone());
        }
        ans.push(title);
        for num in numbers {
            let mut cur = vec![num.to_string()];
            for s in &menu {
                if let Some(v) = hash.get(s) {
                    let many = *v.get(&num).unwrap_or(&0);
                    cur.push(many.to_string());
                }
            }
            ans.push(cur);
        }
        ans
    }
}

struct Solution;