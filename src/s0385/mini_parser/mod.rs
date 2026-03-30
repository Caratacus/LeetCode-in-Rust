// Problem 0385: mini parser

use crate::common::nested_integer::NestedInteger;

pub struct Solution;

impl Solution {
    pub fn is_integer() -> bool {
        todo!()
    }

    pub fn get_integer() -> i32 {
        todo!()
    }

    pub fn set_integer(value: i32) -> () {
        todo!()
    }

    pub fn add(ni: NestedInteger) -> () {
        todo!()
    }

    pub fn get_list() -> Vec<NestedInteger> {
        todo!()
    }

    pub fn deserialize(s: String) -> NestedInteger {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void deserialize()
    //   assertThat(new Solution().deserialize("324").getInteger(), equalTo(324));
    #[test]
    fn test_deserialize() {
        // TODO: 翻译 Java 测试
    }

    // Java: void deserialize2()
    //   NestedInteger nestedInteger = new Solution().deserialize("[123,[456,[789]]]");
    //   int[] result =
    //   new int[] {
    //   nestedInteger.getList().get(0).getInteger(),
    //   nestedInteger.getList().get(1).getList().get(0).getInteger(),
    //   ... (4 more lines)
    #[test]
    fn test_deserialize2() {
        // TODO: 翻译 Java 测试
    }
}
