// Problem 0720: longest word in dictionary
use std::cell::RefCell;
use std::rc::Rc;

use crate::common::graph_node::GraphNode;

pub struct Solution;

impl Solution {
    pub fn insert(curr: Option<Rc<RefCell<GraphNode>>>, word: String) -> () {
        todo!()
    }

    pub fn longest_word(words: Vec<String>) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void longestWord()
    //   assertThat(
    //   new Solution().longestWord(new String[] {"w", "wo", "wor", "worl", "world"}),
    //   equalTo("world"));
    #[test]
    fn test_longest_word() {
        // TODO: 翻译 Java 测试
    }

    // Java: void longestWord2()
    //   assertThat(
    //   new Solution()
    //   .longestWord(
    //   new String[] {
    //   "a", "banana", "app", "appl", "ap", "apply", "apple"
    //   ... (2 more lines)
    #[test]
    fn test_longest_word2() {
        // TODO: 翻译 Java 测试
    }
}
