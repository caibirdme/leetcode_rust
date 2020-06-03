impl Solution {
    pub fn max_area(h: i32, w: i32, mut horizontal_cuts: Vec<i32>, mut vertical_cuts: Vec<i32>) -> i32 {
        horizontal_cuts.push(0);
        horizontal_cuts.push(h);
        horizontal_cuts.sort();
        vertical_cuts.push(0);
        vertical_cuts.push(w);
        vertical_cuts.sort();
        let mut dif_h = 1;
        for i in 1..horizontal_cuts.len() {
            dif_h = dif_h.max(horizontal_cuts[i]-horizontal_cuts[i-1]);
        }
        let mut dif_w = 1;
        for i in 1..vertical_cuts.len() {
            dif_w = dif_w.max(vertical_cuts[i]-vertical_cuts[i-1]);
        }
        const MOD: i64 = 1000000007;
        ((dif_w as i64 * dif_h as i64) % MOD) as i32
    }
}

struct Solution;
