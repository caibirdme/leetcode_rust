// Definition for a binary tree node.
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
    pub fn vertical_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut hash: HashMap<i32, Vec<(i32,usize)>> = HashMap::new();
        Self::dfs(&root, 0, 0, &mut hash);
        let mut indices: Vec<i32> = hash.keys().map(|v| *v).collect();
        indices.sort();
        let mut ans = Vec::with_capacity(indices.len());
        for k in indices {
            let mut v = hash.get(&k).unwrap().clone();
            v.sort_by(|&(_,x),&(_,y)| x.cmp(&y));
            ans.push(v.into_iter().map(|(v,_)| v).collect());
        }
        ans
    }
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, x: i32, h: usize, hash: &mut HashMap<i32, Vec<(i32,usize)>>) {
        if root.is_none() {
            return;
        }
        let br = root.as_ref().unwrap().borrow();
        hash.entry(x).or_insert(vec![]).push((br.val,h));
        let lch = &br.left;
        let rch = &br.right;
        Self::dfs(lch, x-1, h+1, hash);
        Self::dfs(rch, x+1, h+1, hash);
    }
}

struct Solution;