impl Solution {
    pub fn group_the_people(mut group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let n = group_sizes.len();
        let mut ans = vec![];
        for i in 0..n {
            if group_sizes[i] == 0 {
                continue;
            }
            if group_sizes[i] == 1 {
                ans.push(vec![i as i32]);
                continue;
            }
            let mut cur = vec![i as i32];
            for j in i+1..n {
                if group_sizes[i] == group_sizes[j] {
                    cur.push(j as i32);
                    group_sizes[j] = 0;
                }
                if cur.len() as i32 == group_sizes[i] {
                    ans.push(cur);
                    break;
                }
            }
        }
        ans
    }
}

struct Solution;

mod tests {

}