
// Definition for singly-linked list.
 #[derive(PartialEq, Eq, Clone, Debug)]
 pub struct ListNode {
   pub val: i32,
   pub next: Option<Box<ListNode>>
 }

 impl ListNode {
   #[inline]
   fn new(val: i32) -> Self {
     ListNode {
       next: None,
       val
     }
   }
 }

impl ListNode {
    pub fn push(self, val: i32) -> Self {
        Self{
            val,
            next: Some(Box::new(self)),
        }
    }
}
impl Solution {
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut stack_long = Self::reverse(l1);
        let mut stack_short = Self::reverse(l2);
        if stack_long.len() < stack_short.len() {
            let mut temp = stack_long;
            stack_long = stack_short;
            stack_short = temp;
        }
        let mut i = stack_long.len()-1;
        while let Some(v) = stack_short.pop() {
            stack_long[i] += v;
            if stack_long[i] >= 10 && i > 0 {
                stack_long[i] -= 10;
                stack_long[i-1] +=1;
            }
            if i > 0 {
                i-=1;
            }
        }
        while i > 0 && stack_long[i] >= 10 {
            stack_long[i] -= 10;
            stack_long[i-1] += 1;
            i-=1;
        }
        i = stack_long.len()-1;
        let mut ans = ListNode::new(stack_long[i]%10);
        if i > 0 {
            i-=1;
        } else {
            if stack_long[0] >= 10 {
                ans = ans.push(1);
            }
            return Some(Box::new(ans));
        }
        while i>=1 {
            ans = ans.push(stack_long[i]);
            i-=1;
        }
        if stack_long[0] >= 10 {
            ans = ans.push(stack_long[0]-10);
            stack_long[0] = 1;
        }
        if stack_long[0] > 0 {
            ans = ans.push(stack_long[0]);
        }
        Some(Box::new(ans))
    }
    fn reverse(mut list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut stack = vec![];
        while let Some(mut data) = list {
            stack.push(data.val);
            list = data.next.take();
        }
        stack
    }
}

struct Solution;

impl ListNode {
    pub fn construct(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut ans = Self::new(nums[0]);
        for c in nums.into_iter().skip(1) {
            ans = ans.push(c);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use super::ListNode;

    #[test]
    fn test_add_two_numbers() {
        let test_cases = vec![
            (vec![9,8,7,6,9,9], vec![1,2,3,4,5,6,7], vec![2,2,2,2,2,6,6]),
            (vec![1], vec![9,9,9], vec![1,0,0,0,]),
            (vec![9], vec![1], vec![1,0]),
            (vec![0], vec![1], vec![1]),
            (vec![0], vec![0], vec![0]),
            (vec![7,2,4,3], vec![5,6,4], vec![7,8,0,7]),
        ];
        for (l1,l2,e) in test_cases {
            let mut x1 = l1.clone();
            x1.reverse();
            let mut x2 = l2.clone();
            x2.reverse();
            let mut n1 = Some(Box::new(ListNode::construct(x1)));
            let mut n2 = Some(Box::new(ListNode::construct(x2)));
            let actual = Solution::add_two_numbers(n1, n2);
            let mut x3 = e.clone();
            x3.reverse();
            let expect = Some(Box::new(ListNode::construct(x3)));
            assert_eq!(expect, actual, "l1: {:?}, l2: {:?}", l1, l2);
        }
    }
}