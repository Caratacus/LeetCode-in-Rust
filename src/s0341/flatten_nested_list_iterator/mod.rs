// Problem 0341: flatten nested list iterator

use crate::common::nested_integer::NestedInteger;

pub struct NestedIterator {
    nested_list: Vec<NestedInteger>,
}

impl NestedIterator {
    pub fn new(nested_list: Vec<NestedInteger>) -> Self {
        todo!()
    }

    pub fn is_integer(&mut self) -> bool {
        todo!()
    }

    pub fn get_integer(&self) -> i32 {
        todo!()
    }

    pub fn get_list(&self) -> Vec<NestedInteger> {
        todo!()
    }

    pub fn next(&mut self) -> i32 {
        todo!()
    }

    pub fn has_next(&mut self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void nestedIterator()
    //   NestedIterator iterator =
    //   new NestedIterator(
    //   Arrays.asList(
    //   new NestedInteger(
    //   Arrays.asList(new NestedInteger(1), new NestedInteger(1))),
    //   ... (9 more lines)
    #[test]
    fn test_nested_iterator() {
        // TODO: 翻译 Java 测试
    }

    // Java: void nestedIterator2()
    //   NestedInteger integer1 = new NestedInteger(1);
    //   NestedInteger integer2 = new NestedInteger(2);
    //   NestedInteger integer3 = new NestedInteger(3);
    //   List<NestedInteger> list = new Vec<>();
    //   list.add(integer1);
    //   ... (14 more lines)
    #[test]
    fn test_nested_iterator2() {
        // TODO: 翻译 Java 测试
    }
}
