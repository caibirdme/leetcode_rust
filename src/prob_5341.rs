enum Item {
    Calc(Vec<i32>),
    Val(i32),
}

struct ProductOfNumbers {
    f: Vec<Item>,
}

impl ProductOfNumbers {

    fn new() -> Self {
        Self{
            f: vec![],
        }
    }

    fn add(&mut self, num: i32) {
        self.f.push(Item::Val(num));
    }

    fn get_product(&mut self, k: i32) -> i32 {
        let mut i = self.f.len()-1;
        let mut uk = k as usize;
        let mut f = vec![1];
        while uk > 0 {
            let mut clear = false;
            while let Item::Val(v) = &self.f[i] {
                f.push(*f.last().unwrap() * *v);
                uk -= 1;
                if i == 0 {
                    clear = true;
                    break;
                } else {
                    i-=1;
                }
                if uk == 0 {
                    break;
                }
            }
            if uk == 0 {
                if clear {
                    self.f.clear();
                }
                let ans = *f.last().unwrap();
                self.f.truncate(i+1);
                self.f.push(Item::Calc(f));
                return ans;
            }
            if let Item::Calc(ref arr) = &self.f[i] {
                if arr.len() > uk {
                    let val = arr[uk];
                    self.f.truncate(i+1);
                    let ans = *f.last().unwrap() * val;
                    self.f.push(Item::Calc(f));
                    return ans;
                } else {
                    let last = *f.last().unwrap();
                    for j in 1..arr.len() {
                        f.push(last * arr[j]);
                    }
                    uk -= arr.len()-1;
                    if i > 0 {
                        i-=1;
                    } else {
                        clear = true;
                    }
                }
            }
            if uk == 0 {
                if clear {
                    self.f.clear();
                }
                let ans = *f.last().unwrap();
                self.f.push(Item::Calc(f));
                return ans;
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_product_1() {
        let mut productOfNumbers = ProductOfNumbers::new();
        productOfNumbers.add(3);        // [3]
        productOfNumbers.add(0);        // [3,0]
        productOfNumbers.add(2);        // [3,0,2]
        productOfNumbers.add(5);        // [3,0,2,5]
        productOfNumbers.add(4);        // [3,0,2,5,4]
        assert_eq!(productOfNumbers.get_product(2), 20); // 返回 20 。最后 2 个数字的乘积是 5 * 4 = 20
        assert_eq!(productOfNumbers.get_product(3), 40); // 返回 40 。最后 3 个数字的乘积是 2 * 5 * 4 = 40
        assert_eq!(productOfNumbers.get_product(4), 0); // 返回  0 。最后 4 个数字的乘积是 0 * 2 * 5 * 4 = 0
        productOfNumbers.add(8);        // [3,0,2,5,4,8]
        assert_eq!(productOfNumbers.get_product(2), 32); // 返回 32 。最后 2 个数字的乘积是 4 * 8 = 32
    }
    #[test]
    fn test_get_product_2() {
        let mut productOfNumbers = ProductOfNumbers::new();
        productOfNumbers.add(1);
        assert_eq!(productOfNumbers.get_product(1), 1);
        assert_eq!(productOfNumbers.get_product(1), 1);
        assert_eq!(productOfNumbers.get_product(1), 1);
        productOfNumbers.add(7);
        productOfNumbers.add(6);
        productOfNumbers.add(7);

    }
}