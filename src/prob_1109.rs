impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let mut bits = vec![0; n as usize+2];
        for booking in bookings {
            let (from,to,val) = (booking[0], booking[1],booking[2]);
            Self::update(&mut bits, from, val);
            Self::update(&mut bits, to+1, -val);
        }
        (1..=n).into_iter().map(|i| Self::query(&bits, i)).collect()
    }
    fn update(bits: &mut Vec<i32>, mut x: i32, v: i32) {
        let n = bits.len() as i32;
        while x < n {
            bits[x as usize] += v;
            x += x&-x;
        }
    }
    fn query(bits: &Vec<i32>, mut x: i32) -> i32 {
        let mut ans = 0;
        while x > 0 {
            ans += bits[x as usize];
            x -= x&-x;
        }
        ans
    }
}

struct Solution;