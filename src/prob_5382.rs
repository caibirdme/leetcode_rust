impl Solution {
    pub fn entity_parser(text: String) -> String {
        if text.is_empty() {
            return text;
        }
        let text = text.as_bytes();
        let mut ans = vec![];
        let quot = "quot;";
        let apos = "apos;";
        let amp = "amp;";
        let gt = "gt;";
        let lt = "lt;";
        let frasl = "frasl;";
        let n = text.len();
        let mut i = 0;
        while i < n {
            if text[i] != b'&' {
                ans.push(text[i]);
                i+=1;
                continue;
            }
            if i+3 < n {
                if text[i+1..i+4].eq(gt.as_bytes()) {
                    ans.push(b'>');
                    i+=4;
                    continue;
                }
                if text[i+1..i+4].eq(lt.as_bytes()) {
                    ans.push(b'<');
                    i+=4;
                    continue;
                }
            }
            if i+4 < n {
                if text[i+1..i+5].eq(amp.as_bytes()) {
                    ans.push(b'&');
                    i+=5;
                    continue;
                }
            }
            if i+5 < n {
                if text[i+1..i+6].eq(quot.as_bytes()) {
                    ans.push(b'"');
                    i+=6;
                    continue;
                }
                if text[i+1..i+6].eq(apos.as_bytes()) {
                    ans.push(b'\'');
                    i+=6;
                    continue;
                }
            }
            if i+6 < n {
                if text[i+1..i+7].eq(frasl.as_bytes()) {
                    ans.push(b'/');
                    i+=7;
                    continue;
                }
            }
            ans.push(b'&');
            i+=1;
        }
        unsafe {std::str::from_utf8_unchecked(&ans).to_string()}
    }
}

struct Solution;