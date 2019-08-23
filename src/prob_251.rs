
struct Vector2D {
    v: Vec<Vec<i32>>,
    x: usize,
    y: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Vector2D {

    fn new(v: Vec<Vec<i32>>) -> Self {
        Self{
            v,
            x:0,
            y:0,
        }
    }

    fn next(&mut self) -> i32 {
        while self.x < self.v.len() && self.v[self.x].is_empty() { self.x += 1; }
        if self.y >= self.v[self.x].len() {
            self.x += 1;
            while self.x < self.v.len() && self.v[self.x].is_empty() { self.x += 1; }
            self.y = 0;
        }
        let t = self.v[self.x][self.y];
        self.y += 1;
        t
    }

    fn has_next(&mut self) -> bool {
        if self.v.is_empty() {
            return false;
        }
        while self.x < self.v.len() && self.v[self.x].is_empty() { self.x += 1; }
        if self.x >= self.v.len() {
            return false;
        }
        if self.y >= self.v[self.x].len() {
            self.x += 1;
            while self.x < self.v.len() && self.v[self.x].is_empty() { self.x += 1; }
            if self.x >= self.v.len() {
                return false;
            }
            self.y = 0;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten_2d_vec() {
        let mut v2d = Vector2D::new(vec![vec![1,2,3], vec![4], vec![], vec![], vec![5,6]]);
        assert_eq!(v2d.has_next(), true);
        assert_eq!(v2d.has_next(), true);
        assert_eq!(v2d.next(), 1);
        assert_eq!(v2d.has_next(), true);
        assert_eq!(v2d.next(), 2);
        assert_eq!(v2d.has_next(), true);
        assert_eq!(v2d.next(), 3);
        assert_eq!(v2d.has_next(), true);
        assert_eq!(v2d.next(), 4);
        assert_eq!(v2d.next(), 5);
        assert_eq!(v2d.has_next(), true);
        assert_eq!(v2d.next(), 6);
        assert_eq!(v2d.has_next(), false);
        v2d = Vector2D::new(vec![vec![]]);
        assert_eq!(v2d.has_next(), false);
    }
}