impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let n = pushed.len();
        if n != popped.len() {
            return false;
        }
        let mut stack = vec![];
        let mut i = 0;
        for &v in &pushed {
            stack.push(v);
            while i < n {
                if let Some(peak) = stack.last() {
                    if *peak == popped[i] {
                        stack.pop();
                        i+=1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        stack.is_empty() && i==n
    }
}

struct Solution;