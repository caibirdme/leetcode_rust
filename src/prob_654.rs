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
 use std::cmp::max;

 struct SegmentTree {
     root: Option<Box<SegTreeNode>>,
     n: usize,
 }

 struct SegTreeNode {
     val: i32,
     idx: usize,
     lch: Option<Box<SegTreeNode>>,
     rch: Option<Box<SegTreeNode>>,
 }

 impl SegTreeNode {
     fn new(arr: &Vec<i32>, l: usize, r: usize) -> SegTreeNode {
         if l == r {
             return SegTreeNode{
                 val: arr[l-1],
                 idx: l,
                 lch: None,
                 rch: None,
             }
         }
         let mid = (l+r)/2;
         let lch = Self::new(arr, l, mid);
         let rch = Self::new(arr, mid+1, r);
         let (v,idx) = {
             if lch.val > rch.val {
                 (lch.val, lch.idx)
             } else {
                 (rch.val, rch.idx)
             }
         };
         SegTreeNode{
             val: v,
             idx,
             lch: Some(Box::new(lch)),
             rch: Some(Box::new(rch)),
         }
     }
     fn get_max(&self, lb: usize, rb:usize, l:usize, r:usize) -> (i32,usize) {
         if l == r || (l == lb && r == rb ){
             return (self.val, self.idx) ;
         }
         let mid = (l+r)/2;
         if rb <= mid {
             self.lch.as_ref().unwrap().get_max(lb, rb, l, mid)
         } else if lb > mid {
             self.rch.as_ref().unwrap().get_max(lb, rb, mid+1, r)
         } else {
             let (lv, l_idx) = self.lch.as_ref().unwrap().get_max(lb, mid, l, mid);
             let (rv, r_idx) = self.rch.as_ref().unwrap().get_max(mid+1, rb, mid+1, r);
            if lv > rv {
                (lv, l_idx)
            } else {
                (rv, r_idx)
            }
         }
     }
 }

 impl SegmentTree {
     fn new(arr: &Vec<i32>) -> Self {
         let root = SegTreeNode::new(arr, 1, arr.len());
         Self{
             root: Some(Box::new(root)),
             n: arr.len(),
         }
     }
     fn get_max(&self, l: usize, r: usize) -> usize {
         let (_, idx) = self.root.as_ref().unwrap().get_max(l,r,1,self.n);
         idx
     }
 }

impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let smt = SegmentTree::new(&nums);
        Self::build(&nums, &smt, 1, nums.len())
    }
    fn build(nums: &Vec<i32>, smt: &SegmentTree, l: usize, r: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if l > r {
            return None;
        }
        if l == r {
            return Some(Rc::new(RefCell::new(TreeNode{
                val: nums[l-1],
                left: None,
                right: None,
            })));
        }
        let idx = smt.get_max(l, r);
        let lch = Self::build(nums, smt, l, idx-1);
        let rch = Self::build(nums, smt, idx+1, r);
        Some(Rc::new(RefCell::new(TreeNode{
            val: arr[idx-1],
            left: lch,
            right: rch,
        })))
    }
}

struct Solution;