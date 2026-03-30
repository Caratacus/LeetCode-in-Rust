// Problem 1357: apply discount every n orders

pub struct Cashier {
    n: i32,
    discount: i32,
    products: Vec<i32>,
    prices: Vec<i32>,
}

impl Cashier {
    pub fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        todo!()
    }

    pub fn get_bill(&self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void cashierTest()
    //   Cashier cashier =
    //   new Cashier(
    //   3,
    //   50,
    //   new int[] {1, 2, 3, 4, 5, 6, 7},
    //   ... (13 more lines)
    #[test]
    fn test_cashier_test() {
        // TODO: 翻译 Java 测试
    }
}
