use std::cmp::{min, max};

struct NumMatrix {
    n: usize,
    m: usize,
    root: Option<Box<TreeNode>>,
}

struct TreeNode {
    sum: i32,
    child: [Option<Box<TreeNode>>; 4],
}

impl TreeNode {
    fn new() -> Self {
        Self{
            sum: 0,
            child: [None,None,None,None],
        }
    }

    fn build(&mut self, matrix: &Vec<Vec<i32>>, lx: usize, ly:usize, rx:usize,ry:usize) {
        if lx == rx && ly == ry {
            self.sum = matrix[lx][ly];
            return;
        }
        let mx = (lx+rx)/2;
        let my = (ly+ry)/2;
        let mut zero = Self::new();
        zero.build(matrix, lx,ly,mx,my);
        self.child[0] = Some(Box::new(zero));
        if rx > lx {
            let mut one = Self::new();
            one.build(matrix, mx+1, ly, rx, my);
            self.child[1] = Some(Box::new(one));
        }
        if ry > ly {
            let mut two = Self::new();
            two.build(matrix, lx,my+1, mx, ry);
            self.child[2] = Some(Box::new(two));
        }
        if rx > lx && ry > ly {
            let mut three = Self::new();
            three.build(matrix, mx+1, my+1, rx, ry);
            self.child[3] = Some(Box::new(three));
        }
        for v in self.child.iter() {
            self.sum += v.as_ref().map_or(0, |a| a.sum);
        }
    }

    fn update(&mut self, lx:usize,ly:usize,rx:usize,ry:usize,row1:usize,col1:usize,val:i32) {
        if lx == rx && ly == ry {
            self.sum = val;
            return;
        }
        let mx = (lx+rx)/2;
        let my = (ly+ry)/2;
        if row1 <= mx {
            if col1 <= my {
                self.sum -= self.child[0].as_ref().unwrap().sum;
                self.child[0].as_mut().unwrap().update(lx,ly,mx,my,row1,col1,val);
                self.sum += self.child[0].as_ref().unwrap().sum;
            }else {
                self.sum -= self.child[2].as_ref().unwrap().sum;
                self.child[2].as_mut().unwrap().update(lx,my+1,mx,ry,row1,col1,val);
                self.sum += self.child[2].as_ref().unwrap().sum;
            }
        } else {
            if col1 <= my {
                self.sum -= self.child[1].as_ref().unwrap().sum;
                self.child[1].as_mut().unwrap().update(mx+1,ly,rx,my,row1,col1,val);
                self.sum += self.child[1].as_ref().unwrap().sum;
            } else {
                self.sum -= self.child[3].as_ref().unwrap().sum;
                self.child[3].as_mut().unwrap().update(mx+1,my+1,rx,ry,row1,col1,val);
                self.sum += self.child[3].as_ref().unwrap().sum;
            }
        }
    }

    fn query_sum(&self, lx:usize,ly:usize,rx:usize,ry:usize,row1:usize,col1:usize,row2:usize,col2:usize) -> i32 {
        if lx == row1 && ly == col1 && rx == row2 && ry == col2 {
            return self.sum;
        }
        let mx = (lx+rx)/2;
        let my = (ly+ry)/2;
        let mut sum = 0;
        // zero
        if row1 <= mx && col1 <= my {
            sum += self.child[0].as_ref().unwrap().query_sum(lx,ly,mx,my,row1,col1,min(row2, mx), min(col2,my));
        }
        // one
        if row2 > mx && col1 <= my {
            sum += self.child[1].as_ref().unwrap().query_sum(mx+1, ly, rx,my, max(mx+1, row1), col1, min(rx,row2), min(my, col2));
        }
        // two
        if row1 <= mx && col2 > my {
            sum += self.child[2].as_ref().unwrap().query_sum(lx,my+1,mx,ry, row1,max(col1,my+1),min(mx,row2),min(ry,col2));
        }
        // three
        if row2>mx && col2>my {
            sum += self.child[3].as_ref().unwrap().query_sum(mx+1,my+1,rx,ry,max(row1,mx+1), max(col1,my+1), min(row2,rx),min(col2,ry));
        }
        sum
    }
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {

    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let n = matrix.len();
        if n == 0 {
            return Self {
                n:0,m:0,root:None,
            };
        }
        let m = matrix[0].len()-1;
        let mut root = TreeNode::new();
        root.build(&matrix, 0,0,n-1,m);
        Self{ n:n-1,m,root:Some(Box::new(root)), }
    }

    fn update(&mut self, row: i32, col: i32, val: i32) {
        if self.root.is_none() {
            return;
        }
        self.root.as_mut().unwrap().update(0,0,self.n,self.m,row as usize,col as usize, val);
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        if self.root.is_none() {
            return 0;
        }
        self.root.as_ref().unwrap().query_sum(0,0,self.n,self.m, row1 as usize, col1 as usize, row2 as usize, col2 as usize)
    }

}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * obj.update(row, col, val);
 * let ret_2: i32 = obj.sum_region(row1, col1, row2, col2);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix() {
        let matrix = vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ];
        let mut nm = NumMatrix::new(matrix);
        assert_eq!(nm.sum_region(0,1,4,3), 24);
        assert_eq!(nm.sum_region(2,1,4,3), 8);
        nm.update(3,2,2);
        assert_eq!(nm.sum_region(2,1,4,3), 10);
        assert_eq!(nm.sum_region(0,0,0,0), 3);
        assert_eq!(nm.sum_region(0,1,4,3), 26);
        assert_eq!(nm.sum_region(0,0,4,0), 14);
        assert_eq!(nm.sum_region(3,4,4,4), 12);
        nm.update(4,2,-1);
        nm.update(4,4,0);
        assert_eq!(nm.sum_region(2,1,4,4), 18);

        let mut emp = NumMatrix::new(vec![]);
        emp.update(1000,1000,1);
        assert_eq!(emp.sum_region(0,0,100,980), 0);
    }
}