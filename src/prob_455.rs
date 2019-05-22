impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();
        let mut ans = 0;
        let (gn,sn) = (g.len(), s.len());
        let (mut i, mut j) = (0,0);
        while i < gn && j < sn {
            while j < sn && s[j] < g[i] {j+=1;}
            if j < sn {
                ans+=1;
                j+=1;
            }
            i+=1;
        }
        ans
    }
}

struct Solution;