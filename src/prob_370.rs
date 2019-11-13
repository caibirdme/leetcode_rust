impl Solution {
    pub fn get_modified_array(length: i32, updates: Vec<Vec<i32>>) -> Vec<i32> {
        let mut arr = vec![0; length as usize+1];
        for update in updates {
            let (i,j,k) = (update[0] as usize, update[1] as usize, update[2]);
            arr[i] += k;
            arr[j+1] -= k;
        }
        for i in 1..length as usize {
            arr[i] += arr[i-1];
        }
        arr.pop();
        arr
    }
}

struct Solution;