// Problem 1048: longest string chain

pub struct Solution;

impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        todo!()
    }

    pub fn find_longest(
        word: String,
        len_str: Vec<Vec<String>>,
        longest: std::collections::HashMap<String, i32>,
    ) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void longestStrChain()
    //   assertThat(
    //   new Solution().longestStrChain(new String[] {"a", "b", "ba", "bca", "bda", "bdca"}),
    //   equalTo(4));
    #[test]
    fn test_longest_str_chain() {
        // TODO: 翻译 Java 测试
    }

    // Java: void longestStrChain2()
    //   assertThat(
    //   new Solution()
    //   .longestStrChain(new String[] {"xbc", "pcxbcf", "xb", "cxbc", "pcxbc"}),
    //   equalTo(5));
    #[test]
    fn test_longest_str_chain2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void longestStrChain3()
    //   assertThat(new Solution().longestStrChain(new String[] {"abcd", "dbqca"}), equalTo(1));
    #[test]
    fn test_longest_str_chain3() {
        // TODO: 翻译 Java 测试
    }
}
