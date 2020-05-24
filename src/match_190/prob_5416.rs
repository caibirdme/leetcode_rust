impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let search = search_word.as_str();
        for (idx, s) in sentence.split(" ").enumerate() {
            if s.starts_with(search) {
                return idx as i32+1;
            }
        }
        return -1;
    }
}

struct Solution;