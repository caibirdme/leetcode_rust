impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut q = vec![];
        for v in asteroids {
            if v > 0 {
                q.push(v);
            } else {
                let weight = -v;
                while let Some(&top) = q.last() {
                    if top < 0 || weight <= top {break;}
                    q.pop();
                }
                if let Some(&top) = q.last() {
                    if top == weight {
                        q.pop();
                    } else if top < 0 {
                        q.push(v);
                    }
                } else {
                    q.push(v);
                }
            }
        }
        q
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_asteroid_collision() {
        let test_cases = vec![
            (vec![5,10,-5], vec![5,10]),
            (vec![8,-8], vec![]),
            (vec![10,2,-5], vec![10]),
        ];
        for (asteroids, expect) in test_cases {
            assert_eq!(Solution::asteroid_collision(asteroids.clone()), expect, "asteroids: {:?}", asteroids);
        }
    }
}