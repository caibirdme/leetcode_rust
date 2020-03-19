use std::collections::HashSet;

impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let mut set = HashSet::new();
        let mut lx = std::i32::MAX;
        let mut ly = std::i32::MAX;
        let mut rx = std::i32::MIN;
        let mut ry = std::i32::MIN;
        let mut area = 0;
        for rec in rectangles {
            let (x1,y1,x2,y2) = (rec[0], rec[1], rec[2], rec[3]);
            area += (x2-x1)*(y2-y1);
            lx = lx.min(x1);
            ly = ly.min(y1);
            rx = rx.max(x2);
            ry = ry.max(y2);
            if !set.contains(&(x1, y1)) {
                set.insert((x1,y1));
            } else {
                set.remove(&(x1, y1));
            }
            if !set.contains(&(x2,y2)) {
                set.insert((x2, y2));
            } else {
                set.remove(&(x2, y2));
            }
            if !set.contains(&(x1,y2)) {
                set.insert((x1, y2));
            } else {
                set.remove(&(x1, y2));
            }
            if !set.contains(&(x2,y1)) {
                set.insert((x2, y1));
            } else {
                set.remove(&(x2, y1));
            }
        }
        if set.len() != 4 ||
            !set.contains(&(lx, ly)) ||
            !set.contains(&(rx, ry)) ||
            !set.contains(&(lx, ry)) ||
            !set.contains(&(rx, ly))
        {
            return false;
        }
        area == (rx-lx)*(ry-ly)
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_is_rectangle_cover() {
        let test_cases = vec![
            (vec![
                vec![0,0,1,1],
                vec![0,0,2,1],
                vec![1,0,2,1],
                vec![0,2,2,3],
            ], false),
            (vec![
                vec![1,1,3,3],
                vec![3,1,4,2],
                vec![3,2,4,4],
                vec![1,3,2,4],
                vec![2,3,3,4],
            ], true),
        ];
        for (rectangles, ok) in test_cases {
            assert_eq!(Solution::is_rectangle_cover(rectangles.clone()), ok, "rectangles: {:?}", rectangles);
        }
    }
}