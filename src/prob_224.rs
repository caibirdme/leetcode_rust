struct Tokenizer<'a> {
    s: &'a[u8],
    idx: usize,
}

impl<'a> Tokenizer<'a> {
    fn new(s: &'a [u8]) -> Self {
        Self{
            s,
            idx:0,
        }
    }
    fn next(&mut self) -> Option<Token> {
        while self.idx < self.s.len() && self.s[self.idx] == b' ' {self.idx+=1;}
        if self.idx >= self.s.len() {
            return None;
        }
        let res = match self.s[self.idx] {
            b'+' => Token::Add,
            b'-' => Token::Sub,
            b'*' => Token::Mul,
            b'/' => Token::Div,
            b'(' => Token::LP,
            b')' => Token::RP,
            _ => {
                let mut t = 0;
                while self.idx < self.s.len() && self.s[self.idx] >= b'0' && self.s[self.idx] <= b'9' {
                    t = t * 10 + (self.s[self.idx]-b'0') as i32;
                    self.idx += 1;
                }
                self.idx -= 1;
                Token::Num(t)
            }
        };
        self.idx+=1;
        Some(res)
    }
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Num(i32),
    Add,
    Sub,
    Mul,
    Div,
    LP,
    RP,
}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        use Token::*;
        let mut nums = vec![];
        let mut ops = vec![];
        let mut tokenizer = Tokenizer::new(s.as_bytes());
        while let Some(token) = tokenizer.next() {
            match token {
                Num(val) => {
                    if let Some(&op) = ops.last() {
                        match op {
                            Div | Mul => {
                                ops.pop();
                                let new_val = Self::calc(nums.pop().unwrap(), val, op);
                                nums.push(new_val);
                            }
                            _ => nums.push(val),
                        }
                    } else {
                        nums.push(val);
                    }
                },
                Add|Sub => {
                    while let Some(&op) = ops.last() {
                        match op {
                            Add|Sub => {
                                ops.pop();
                                Self::pop_calc_push(&mut nums, op);
                            },
                            _ => {
                                break;
                            }
                        }
                    }
                    ops.push(token);
                },
                Mul|Div|LP => ops.push(token),
                RP => {
                    while let Some(op) = ops.pop() {
                        match op {
                            LP => break,
                            _ => Self::pop_calc_push(&mut nums, op),
                        }
                    }
                    while let Some(&op) = ops.last() {
                        match op {
                            Mul|Div => {
                                ops.pop();
                                Self::pop_calc_push(&mut nums, op);
                            }
                            _ => break,
                        }
                    }
                }
            }
        }
        while let Some(op) = ops.pop() {
            Self::pop_calc_push(&mut nums, op);
        }
        nums.pop().unwrap()
    }
    fn calc(a: i32, b: i32, op: Token) -> i32 {
        use Token::*;
        match op {
            Add => a+b,
            Sub => a-b,
            Mul => a*b,
            Div => a/b,
            _ => unreachable!(),
        }
    }
    fn pop_calc_push(nums: &mut Vec<i32>, op: Token) {
        let b = nums.pop().unwrap();
        let a = nums.pop().unwrap();
        nums.push(Self::calc(a,b,op));
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_calculate() {
        let test_cases = vec![
            ("2-3-(1+5)", -7),
            ("2+10/5*((4-1)*3)+2                  ", 22),
            ("2*(3+(10-4)/2)+1", 13),
            ("10+100/2/2*(4-5)",-15),
            ("10+100/2/2*4-5",105),
            ("100/2/2*4+5",105),
            ("2-  1+4", 5),
            ("1  +  1", 2),
            ("(1+(4+5+2)-3)+(6+8)", 23),
            ("7+8/4+2-2*3", 5),
        ];
        for (s, expect) in test_cases {
            assert_eq!(Solution::calculate(s.to_string()), expect, "s: {}", s);
        }
    }
}