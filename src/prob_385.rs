 #[derive(Debug, PartialEq, Eq)]
 pub enum NestedInteger {
   Int(i32),
   List(Vec<NestedInteger>)
 }
impl Solution {
    pub fn deserialize(mut s: String) -> NestedInteger {
        if s.len() == 0 {
            return NestedInteger::Int(0);
        }
        let mut parser = Parser::new(s.as_bytes());
        parser.parse()
    }
}

 struct Parser<'a> {
     buf: &'a [u8],
     idx: usize,
 }

 impl<'a> Parser<'a> {
     pub fn new(s: &'a [u8]) -> Self {
         Self{
             buf: s,
             idx: 0,
         }
     }
     fn is_num(c: u8) -> bool {
         match c {
             b'0'...b'9' => true,
             b'-' => true,
             _ => false,
         }
     }
     pub fn parse(&mut self) -> NestedInteger {
         match self.buf[0] {
             b'[' => self.parse_vec(),
             _ => self.parse_int(),
         }
     }
     fn parse_vec(&mut self) -> NestedInteger {
         // eat [
         self.idx += 1;
         let mut res = vec![];
         while self.buf[self.idx] != b']' {
             match self.buf[self.idx] {
                 b'[' => {res.push(self.parse_vec());},
                 b',' => {self.idx+=1;},
                 _ => {res.push(self.parse_int());}
             }
         }
         // eat ]
         self.idx+=1;
         NestedInteger::List(res)
     }
     fn parse_int(&mut self) -> NestedInteger {
         let mut minus = 1;
         if self.buf[self.idx] == b'-' {
             minus = -1;
             self.idx+=1;
         }
         let s = self.idx;
         let mut pos = s;
         let mut num = 0;
         while pos < self.buf.len() && Self::is_num(self.buf[pos]) {
             num = num*10 + (self.buf[pos]-b'0') as i32;
             pos+=1;
         }
         self.idx = pos;
         NestedInteger::Int(num*minus)
     }
 }

struct Solution;

 #[cfg(test)]
 mod tests {
     use super::*;
     #[test]
     fn test_deserialize() {
         let test_cases = vec![
             ("324", NestedInteger::Int(324)),
             ("-324", NestedInteger::Int(-324)),
             ("[123,[456,[789]]]", NestedInteger::List(vec![
                NestedInteger::Int(123),NestedInteger::List(vec![
                    NestedInteger::Int(456), NestedInteger::List(vec![NestedInteger::Int(789)])
                 ])
             ])),
             ("[123,[456,[789]],-900]", NestedInteger::List(vec![
                 NestedInteger::Int(123),
                 NestedInteger::List(vec![
                     NestedInteger::Int(456), NestedInteger::List(vec![NestedInteger::Int(789)])
                 ]),
                 NestedInteger::Int(-900),
             ])),
             ("[0,-0,-1,2]", NestedInteger::List(vec![
                 NestedInteger::Int(0),
                 NestedInteger::Int(0),
                 NestedInteger::Int(-1),
                 NestedInteger::Int(2),
             ])),
             ("[-123123]", NestedInteger::List(vec![
                 NestedInteger::Int(-123123),
             ])),
         ];
         for (s, expect) in test_cases {
             assert_eq!(expect, Solution::deserialize(s.to_string()), "s: {}", s);
         }
     }
 }