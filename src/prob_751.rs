impl Solution {
    pub fn ip_to_cidr(ip: String, n: i32) -> Vec<String> {
        if n == 1 {
            return vec![ip+"/32"];
        }
        let mut ans = vec![];
        let mut start = Self::ip_2_int(&ip);
        let mut n = n as usize;
        while n > 0 {
            let mut v = if start == 0 {1} else {(start^(start-1))&start};
            while v > n { v >>= 1;}
            let mut t = v;
            let mut step = 0;
            while t > 1 {
                t >>= 1;
                step+=1;
            }
            ans.push(Self::int_2_ip(start) + "/" + (32-step).to_string().as_str());
            start += v;
            n -= v;
        }
        ans
    }
    fn int_2_ip(mut ip: usize) -> String {
        let mut arr = vec![0; 4];
        for i in (0..4usize).rev() {
            arr[i] = ip & 0xffusize;
            ip >>= 8;
        }
        arr.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(".")
    }
    fn ip_2_int(ip: &String) -> usize {
        let bs = ip.as_bytes();
        let n = bs.len();
        let mut i = 0;
        let mut ans = 0;
        let mut t = 24;
        while i < bs.len() {
            let mut j = i;
            let mut acc = 0usize;
            while j < n && bs[j] != b'.' {
                acc = acc*10 + (bs[j]-b'0') as usize;
                j+=1;
            }
            ans |= acc << t;
            t -= 8;
            i = j+1;
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_ip_to_cidr() {
        let test_cases = vec![
            ("0.0.0.0", 5, vec!["0.0.0.0/32", "0.0.0.1/32", "0.0.0.2/31", "0.0.0.4/32"],),
            ("255.0.0.7", 10, vec!["255.0.0.7/32","255.0.0.8/29","255.0.0.16/32"]),
        ];
        for (ip, n, expect) in test_cases {
            assert_eq!(Solution::ip_to_cidr(ip.to_string(), n), expect, "ip: {}, n: {}", ip, n);
        }
    }
}