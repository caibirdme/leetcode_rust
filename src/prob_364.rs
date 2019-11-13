use std::cmp::max;

 #[derive(Debug, PartialEq, Eq)]
 pub enum NestedInteger {
   Int(i32),
   List(Vec<NestedInteger>)
 }


impl Solution {
    pub fn depth_sum_inverse(nested_list: Vec<NestedInteger>) -> i32 {
        let mut max_depth = 1;
        Self::get_max_depth(&nested_list, 1, &mut max_depth);
        let mut total = 0;
        Self::calc(&nested_list, max_depth, &mut total);
        total
    }
    fn get_max_depth(nested_list: &Vec<NestedInteger>, dep: i32, max_depth: &mut i32) {
        *max_depth = max(*max_depth, dep);
        nested_list.iter().for_each(|v| {
            match v {
                NestedInteger::Int(_) => {},
                NestedInteger::List(arr) => {
                    Self::get_max_depth(arr, dep+1, max_depth);
                }
            }
        })
    }
    fn calc(nested_list: &Vec<NestedInteger>, dep: i32, total: &mut i32) {
        nested_list.iter().for_each(|v| {
            match v {
                NestedInteger::Int(value) => {
                    *total += *value*dep;
                },
                NestedInteger::List(arr) => {
                    Self::calc(arr, dep-1,total);
                }
            }
        })
    }
}

struct Solution;