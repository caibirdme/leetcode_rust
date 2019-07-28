impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut last:[Option<usize>; 2] = [None,None];

        let mut i = 0;
        while i < nums.len() {
            let v = nums[i];
            if v == 2 {
                i+=1;
                continue;
            }
            if i == 0 {
                last[v as usize] = Some(0);
                i+=1;
                continue;
            }
            if v == nums[i-1] {
                last[v as usize] = Some(i);
                i+=1;
                continue;
            }
            if v == 0 {
                if let Some(last_0_pos) = last[0] {
                    match last[1] {
                        Some(last_1_pos) => {
                            if last_1_pos == last_0_pos + 1 {
                                last[1] = None;
                            }
                            nums.swap(i, last_0_pos+1);
                            last[0] = Some(last_0_pos+1);
                            continue;
                        },
                        None => {
                            nums.swap(last_0_pos+1, i);
                            last[0] = Some(last_0_pos+1);
                            continue;
                        }
                    }
                } else {
                    match last[1] {
                        Some(last_1_pos) => {
                            if last_1_pos == 0 {
                                last[1] = None;
                            }
                            last[0] = Some(0);
                            nums.swap(0, i);
                            continue;
                        },
                        None => {
                            nums.swap(0, i);
                            last[0] = Some(0);
                            i+=1;
                            continue;
                        }
                    }
                }
            } else {
                // v == 1
                if let Some(last_1_pos) = last[1] {
                    let next_1_pos = last_1_pos+1;
                    nums.swap(next_1_pos, i);
                    last[1] = Some(next_1_pos);
                } else {
                    match last[0] {
                        Some(last_0_pos) => {
                            let next_1_pos = last_0_pos+1;
                            nums.swap(i, next_1_pos);
                            last[1] = Some(next_1_pos);
                        },
                        None => {
                            nums.swap(0, i);
                            last[1] = Some(0);
                        }
                    }
                }
                i+=1;
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_sort_colors() {
        let test_cases = vec![
            (vec![0,2,1,2,0,1], vec![0,0,1,1,2,2]),
            (vec![0], vec![0]),
            (vec![1], vec![1]),
            (vec![2], vec![2]),
            (vec![0,1,2], vec![0,1,2]),
            (vec![2,1,0], vec![0,1,2]),
            (vec![1,2,1,0,2,1,0], vec![0,0,1,1,1,2,2]),
            (vec![2,0,2,1,1,0], vec![0,0,1,1,2,2]),
            (vec![2,2,2,1,1,0], vec![0,1,1,2,2,2]),
            (vec![2,1,0,2,1,1,0,1,2], vec![0,0,1,1,1,1,2,2,2]),
        ];
        for (mut nums, expect) in test_cases {
            let cache = nums.clone();
            Solution::sort_colors(&mut nums);
            assert_eq!(nums, expect, "nums: {:?}", cache);
        }
    }
}