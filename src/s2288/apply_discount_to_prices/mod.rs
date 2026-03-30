// Problem 2288: apply discount to prices

pub struct Solution;

impl Solution {
    pub fn discount_prices(sentence: String, discount: i32) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void discountPrices()
    //   assertThat(
    //   new Solution().discountPrices("there are $1 $2 and 5$ candies in the shop", 50),
    //   equalTo("there are $0.50 $1.00 and 5$ candies in the shop"));
    #[test]
    fn test_discount_prices() {
        // TODO: 翻译 Java 测试
    }

    // Java: void discountPrices2()
    //   assertThat(
    //   new Solution().discountPrices("1 2 $3 4 $5 $6 7 8$ $9 $10$", 100),
    //   equalTo("1 2 $0.00 4 $0.00 $0.00 7 8$ $0.00 $10$"));
    #[test]
    fn test_discount_prices2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void discountPrices3()
    //   assertThat(
    //   new Solution().discountPrices("$76111 ab $6 $", 48),
    //   equalTo("$39577.72 ab $3.12 $"));
    #[test]
    fn test_discount_prices3() {
        // TODO: 翻译 Java 测试
    }
}
