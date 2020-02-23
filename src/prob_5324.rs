use std::collections::HashMap;

struct Cashier {
    product_price: HashMap<i32, i32>,
    n: i32,
    discount: i32,
    count: i32,
}

impl Cashier {

    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        let mut hash = HashMap::new();
        for (&pid, &price) in products.iter().zip(prices.iter()) {
            hash.insert(pid, price);
        }
        Self{
            n,
            discount,
            product_price: hash,
            count: 0,
        }
    }

    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        self.count += 1;
        let mut total_price: i64 = 0;
        for (pid, &total) in product.iter().zip(amount.iter()) {
            total_price += *self.product_price.get(pid).unwrap() as i64 * total as i64;
        }
        if self.count == self.n {
            self.count = 0;
            (1f64 - self.discount as f64 / 100f64) * total_price as f64
        } else {
            total_price as f64
        }
    }
}