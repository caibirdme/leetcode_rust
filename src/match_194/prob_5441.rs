use std::collections::HashMap;

impl Solution {
    pub fn get_folder_names(names: Vec<String>) -> Vec<String> {
        let mut dict: HashMap<String, i32> = HashMap::new();
        let mut ans = vec![];
        let n = names.len();
        for name in names {
            if !dict.contains_key(&name) {
                ans.push(name.clone());
                dict.insert(name, 0);
            } else {
                let mut v = *dict.get(&name).unwrap();
                let mut s = String::new();
                loop {
                    v += 1;
                    s = format!("{}({})", name, v);
                    let contain = dict.contains_key(&s);
                    if !contain {
                        break;
                    }
                }
                *dict.get_mut(&name).unwrap() = v;
                dict.insert(s.clone(), 0);
                ans.push(s);
            }
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_get_folder_names() {
        let test_cases = vec![
            (vec!["kingston(0)","kingston","kingston"], vec!["kingston(0)","kingston","kingston(1)"]),
            (vec!["kaido","kaido(1)","kaido","kaido(1)"], vec!["kaido","kaido(1)","kaido(2)","kaido(1)(1)"]),
            (vec!["pes","fifa","gta","pes(2019)"], vec!["pes","fifa","gta","pes(2019)"]),
            (vec!["gta","gta(1)","gta","avalon"], vec!["gta","gta(1)","gta(2)","avalon"]),
            (vec!["onepiece","onepiece(1)","onepiece(2)","onepiece(3)","onepiece"], vec!["onepiece","onepiece(1)","onepiece(2)","onepiece(3)","onepiece(4)"]),
            (vec!["wano","wano","wano","wano"], vec!["wano","wano(1)","wano(2)","wano(3)"]),
        ];
        for (names, expect) in test_cases {
            let c = names.iter().map(|v| v.to_string()).collect();
            assert_eq!(Solution::get_folder_names(c), expect, "names: {:?}", names);
        }
    }
}