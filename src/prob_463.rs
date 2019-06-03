/*
You are given a map in form of a two-dimensional
integer grid where 1 represents land and 0 represents water.

Grid cells are connected horizontally/vertically (not diagonally).
The grid is completely surrounded by water,
and there is exactly one island (i.e., one or more connected land cells).

The island doesn't have "lakes"
(water inside that isn't connected to the water around the island).
One cell is a square with side length 1.
The grid is rectangular, width and height don't exceed 100.
Determine the perimeter of the island.
*/

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut perimeter = 0;
        let n = grid.len();
        if n == 0 {
            return 0;
        }
        let m = grid[0].len();
        if m == 0 {
            return 0;
        }
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    if i == 0 || grid[i-1][j] == 0{
                        perimeter+=1;
                    }
                    if i+1 == n || grid[i+1][j] == 0{
                        perimeter += 1;
                    }
                    if j == 0 || grid[i][j-1] == 0{
                        perimeter += 1;
                    }
                    if j+1==m || grid[i][j+1] == 0{
                        perimeter+=1;
                    }
                }
            }
        }
        perimeter
    }
}

struct Solution;