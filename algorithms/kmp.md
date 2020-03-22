KMP是一个非常常见的字符串匹配算法, 以前也**理解**了多次, 也没少写KMP的代码.
但是, 久了不写又忘了...还是理解不深刻啊.在181周赛的最后一题, 一眼就看出来是求KMP的next数组,
但是有点忘了, 于是不想写了.

趁着比赛完, 我花了很多时间又重新复习了下KMP, 这次我应该比较有信心地说, 我掌握了!

KMP我觉得其实是一个非常典型的原本思想和实现都很简单的算法, 被理论搞得很复杂的东西.

先说KMP的核心思想. 当我们在用s串去匹配模式串p时, 假如已经有m个字符匹配上了,但是在第m+1个字符的位置不匹配, 比如
```
A B C D A B C X .........
A B C D A B C Y
              |
``` 
这时朴素的方法是把p串右移一位, 从头开始匹配. 这是朴素做法. 而KMP只是基于朴素做法的一个优化, 而且思想很简单

由于我们前面已经匹配了m个字符, 假如这m个字符中,前i个字符和后i个字符是一样的, 比如
```
 A B C  Q X  A B C  Q ...
(A B C) Q X (A B C) W ..
       \            |
          \   
            (A B C) Q X (A B C) W
```
这时候我们直接把p串整体右移, 然后接着匹配, 而不需要从头重新开始匹配.

知道了这个优化思路之后, 我们的问题就是, 怎么维护这个公共前缀后缀呢?

即, 我需要一个数据结构, 当字符串p和第i个位置和s[j]不匹配时, 我怎么快速获取以p[i]结尾的最长公共前缀呢? 这其实就是各种资料里讲的维护一个next数组

求next数组其实有点像动态规划. 因为我们本质上是在找公共的前缀和后缀. 假如我们的p串形如
```
A ....
```
如果存在公共的前缀和后缀, 那么公共后缀的开头也必须是A, 即
```
A ..... A 
```
此时next[i] = 0(第一个A的位置). 然后我们就可以接着向后扫描, 假如是:
```
      A B C .... A B C
next:            0 1 2
``` 
我们的next就应该是这样. 具体实现其实很简单, 我们可以用双指针法, 我们设j指向**已经匹配了的前缀**, j指向当前元素(后缀).
一开始j=-1, 表面没有匹配任何前缀. i=0. 然后每次比较p[j+1]和p[i+1]是否相等, 如果相等, p[i+1]显然是一个合法的后缀, 那么我们就同时移动两个指针
如果不相等, 我们可以利用前面求出来的next来重置j, j=next[j].

```rust
pub struct Matcher {
    next: Vec<i32>,
    p: Vec<u8>,
}

impl Matcher {
    pub fn new(p: &[u8]) -> Self {
        let n = p.len();
        let mut next = vec![-1; n];
        let mut i = 0;
        let mut j = -1;
        while i+1 < n {
            if p[(i+1) as usize] == p[(j+1) as usize] {
                i+=1;
                j+=1;
                next[i as usize] = j;
            } else if j == -1 { // 如果当前元素不等于p[0], 显然它不可能有公共前缀
                i+=1;
            } else {
                j = next[j as usize];
            }
        }
        Self{next, p: Vec::from(p),}
    }
    fn do_match(&self, s: &[u8]) -> Vec<usize> {
        if self.p.is_empty() {
            return vec![];
        }
        let mut ans = vec![];
        let mut i = 0;
        let mut j = 0;
        let pn = self.p.len();
        while i < s.len() {
            if s[i] == self.p[j] {
                i+=1;
                j+=1;
                if j == pn {
                    ans.push(i-pn);
                    j = (self.next[j-1]+1) as usize;
                }
            } else if j == 0 {
                i+=1;
            } else {
                j = (self.next[j-1]+1) as usize;
            }
        }
        ans
    }
}
``` 
