// Problem 0648: replace words
use std::cell::RefCell;
use std::rc::Rc;

use crate::common::graph_node::GraphNode;

pub struct Solution;

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        todo!()
    }

    pub fn contains_key(ch: char) -> bool {
        todo!()
    }

    pub fn put(ch: char, node: Option<Rc<RefCell<GraphNode>>>) -> () {
        todo!()
    }

    pub fn get(ch: char) -> Option<Rc<RefCell<GraphNode>>> {
        todo!()
    }

    pub fn is_word_completed() -> bool {
        todo!()
    }

    pub fn set_word_completed(flag: bool) -> () {
        todo!()
    }

    pub fn insert(word: String) -> () {
        todo!()
    }

    pub fn get_root_for_word(word: String) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void replaceWords()
    //   assertThat(
    //   new Solution()
    //   .replaceWords(
    //   Arrays.asList("cat", "bat", "rat"),
    //   "the cattle was rattled by the battery"),
    //   ... (1 more lines)
    #[test]
    fn test_replace_words() {
        // TODO: 翻译 Java 测试
    }

    // Java: void replaceWords2()
    //   assertThat(
    //   new Solution()
    //   .replaceWords(Arrays.asList("a", "b", "c"), "aadsfasf absbs bbab cadsfafs"),
    //   equalTo("a a b c"));
    #[test]
    fn test_replace_words2() {
        // TODO: 翻译 Java 测试
    }
}
