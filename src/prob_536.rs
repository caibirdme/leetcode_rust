 #[derive(Debug, PartialEq, Eq)]
 pub struct TreeNode {
   pub val: i32,
   pub left: Option<Rc<RefCell<TreeNode>>>,
   pub right: Option<Rc<RefCell<TreeNode>>>,
 }

 impl TreeNode {
   #[inline]
   pub fn new(val: i32) -> Self {
     TreeNode {
       val,
       left: None,
       right: None
     }
   }
 }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn str2tree(s: String) -> Option<Rc<RefCell<TreeNode>>> {
        Self::convert(s.as_bytes())
    }
    fn convert(s: &[u8]) -> Option<Rc<RefCell<TreeNode>>> {
        if s.is_empty() {
            return None;
        }
        let (v, len) = Self::get_num(s);
        if len == s.len() {
            return Some(Rc::new(RefCell::new(TreeNode{
                val: v,
                left: None,
                right: None,
            })))
        }
        let mut count = 1;
        let mut i = len+1;
        while count > 0 {
            if s[i] == b'(' {
                count += 1;
            } else if s[i] == b')' {
                count -= 1;
            }
            i+=1;
        }
        let lch = Self::convert(&s[len+1..i-1]);
        let mut rch = None;
        if i < s.len() && s[i] == b'(' {
            rch = Self::convert(&s[i+1..s.len()-1]);
        }
        Some(Rc::new(RefCell::new(TreeNode{
            val: v,
            left: lch,
            right: rch,
        })))
    }
    fn get_num(s: &[u8]) -> (i32, usize) {
        if s[0] == b'-' {
            let (v,len) = Self::eat(&s[1..]);
            (-v, len+1)
        } else {
            Self::eat(s)
        }
    }
    fn eat(s: &[u8]) -> (i32, usize) {
        let mut i = 0;
        let mut v = 0;
        while i < s.len() && s[i] >= b'0' && s[i] <= b'9' {
            v = v*10+((s[i]-b'0') as i32);
            i+=1;
        }
        (v, i)
    }
}

struct Solution;