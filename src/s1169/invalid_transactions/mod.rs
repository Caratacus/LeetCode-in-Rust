// Problem 1169: invalid transactions

pub struct Solution;

impl Solution {
    pub fn invalid_transactions(input: Vec<String>) -> Vec<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void invalidTransactions()
    //   assertThat(
    //   new Solution()
    //   .invalidTransactions(
    //   new String[] {"alice,20,800,mtv", "alice,50,100,beijing"}),
    //   equalTo(Arrays.asList("alice,20,800,mtv", "alice,50,100,beijing")));
    #[test]
    fn test_invalid_transactions() {
        // TODO: 翻译 Java 测试
    }

    // Java: void invalidTransactions2()
    //   assertThat(
    //   new Solution()
    //   .invalidTransactions(
    //   new String[] {"alice,20,800,mtv", "alice,50,1200,mtv"}),
    //   equalTo(Collections.singletonList("alice,50,1200,mtv")));
    #[test]
    fn test_invalid_transactions2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void invalidTransactions3()
    //   assertThat(
    //   new Solution()
    //   .invalidTransactions(new String[] {"alice,20,800,mtv", "bob,50,1200,mtv"}),
    //   equalTo(Collections.singletonList("bob,50,1200,mtv")));
    #[test]
    fn test_invalid_transactions3() {
        // TODO: 翻译 Java 测试
    }
}
