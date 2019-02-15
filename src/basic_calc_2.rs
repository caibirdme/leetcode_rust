impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut scanner = Scanner::new(s.as_bytes());
        let mut digit = vec![];
        let mut ops:Vec<Token> = vec![];
        loop {
            let t = scanner.next_token();
            if t.is_eof() {
                break;
            }
            if t.is_num() {
                digit.push(t.get_num());
            } else {
                while !ops.is_empty() && t.level() <= ops.last().unwrap().level() {
                    let (b,a) = (digit.pop().unwrap(), digit.pop().unwrap());
                    match ops.pop().unwrap() {
                        Token::Add => { digit.push(a+b); },
                        Token::Sub => { digit.push(a-b);},
                        Token::Mul => { digit.push(a*b);},
                        Token::Div => { digit.push(a/b);},
                        _ => unreachable!(),
                    }
                }
                ops.push(t);
            }
        }
        while !ops.is_empty() {
            let (b,a) = (digit.pop().unwrap(), digit.pop().unwrap());
            match ops.pop().unwrap() {
                Token::Add => { digit.push(a+b); },
                Token::Sub => { digit.push(a-b);},
                Token::Mul => { digit.push(a*b);},
                Token::Div => { digit.push(a/b);},
                _ => unreachable!(),
            }
        }
        digit.pop().unwrap()
    }
}

struct Solution;

//expr: term ((+|-) term)*
//term: fac ((*|/) fac)*
//fac: NUM | expr

struct Scanner<'a> {
    s: &'a [u8],
    idx: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(s: &'a [u8]) -> Self {
        Self{s, idx:0,}
    }
    pub fn next_token(&mut self) -> Token {
        while self.idx < self.s.len() && self.s[self.idx] == b' ' { self.idx += 1; }
        if self.idx >= self.s.len() {
            return Token::EOF;
        }
        let c = self.s[self.idx];
        match c {
            b'+' => {
                self.idx+=1;
                return Token::Add;
            },
            b'-' => {
                self.idx+=1;
                return Token::Sub;
            },
            b'*' => {
                self.idx+=1;
                return Token::Mul;
            },
            b'/' => {
                self.idx+=1;
                return Token::Div;
            },
            _ => {
                let s = self.idx;
                self.idx += 1;
                while self.idx < self.s.len() && self.s[self.idx] >= b'0' && self.s[self.idx] <= b'9' {
                    self.idx += 1;
                }
                use std::str;
                let val: i32 = str::from_utf8(&self.s[s..self.idx]).unwrap().parse().unwrap();
                return Token::Num(val);
            },
        }
        unreachable!()
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Token {
    EOF,
    Num(i32),
    Add,
    Sub,
    Mul,
    Div,
}

impl Token {
    fn is_eof(&self) -> bool {
        match self {
            Token::EOF => true,
            _ => false,
        }
    }
    fn is_num(&self) -> bool {
        match self {
            Token::Num(_) => true,
            _ => false,
        }
    }
    fn get_num(&self) -> i32 {
        match self {
            Token::Num(val) => *val,
            _ => 0,
        }
    }
    fn level(&self) -> usize {
        match self {
            Token::Div|Token::Mul => 2,
            Token::Sub|Token::Add => 1,
            _ => 0,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate() {
        assert_eq!(
            Solution::calculate("2 +  100/2/5/2*3-7".to_string()), 10
        );
        assert_eq!(
            Solution::calculate(" 3/2 ".to_string()), 1
        );
        assert_eq!(
            Solution::calculate("3+2*2".to_string()), 7
        );

        assert_eq!(
            Solution::calculate(" 3+5 / 2 ".to_string()), 5
        );
    }
}