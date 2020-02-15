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
 use std::collections::HashMap;

 impl Solution {
    pub fn find_duplicate_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut count = HashMap::new();
        let mut uid = HashMap::new();
        let mut ans = vec![];
        Self::dfs(&root, &mut count, &mut uid, &mut ans);
        ans
    }
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, count: &mut HashMap<i32, i32>, uid: &mut HashMap<String, i32>, ans: &mut Vec<Option<Rc<RefCell<TreeNode>>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let br = root.as_ref().unwrap().borrow();
        let lch = Self::dfs(&br.left, count, uid, ans);
        let rch = Self::dfs(&br.right, count, uid, ans);
        let hashKey = format!("{},{},{}", br.val, lch, rch);
        match uid.get(&hashKey) {
            Some(id) => {
                if let Some(v) = count.get_mut(id) {
                    *v += 1;
                    if *v == 2 {
                        ans.push(Some(root.as_ref().unwrap().clone()));
                    }
                }
                *id
            },
            None => {
                let id = uid.len() as i32+1;
                uid.insert(hashKey.clone(), id);
                count.insert(id, 1);
                id
            }
        }
    }
}

struct Solution;