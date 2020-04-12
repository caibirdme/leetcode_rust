impl Solution {
    pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        let m = m as usize;
        let mut arr = vec![0; m];
        for i in 0..m {
            arr[i] = (i+1) as i32;
        }
        let mut ans = vec![];
        for q in queries {
            for i in 0..m {
                if arr[i] == q {
                    ans.push(i as i32);
                    for j in (0..i).rev() {
                        arr[j+1] = arr[j];
                    }
                    arr[0] = q;
                }
            }
        }
        ans
    }
}

struct Solution;