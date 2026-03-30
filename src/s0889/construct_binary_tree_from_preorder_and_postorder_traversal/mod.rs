// Problem 0889: construct binary tree from preorder and postorder traversal

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void constructFromPrePost()
    //   assertThat(
    //   new Solution()
    //   .constructFromPrePost(
    //   new int[] {1, 2, 4, 5, 3, 6, 7}, new int[] {4, 5, 2, 6, 7, 3, 1})
    //   .toString(),
    //   ... (1 more lines)
    #[test]
    fn test_construct_from_pre_post() {
        // TODO: 翻译 Java 测试
    }

    // Java: void constructFromPrePost2()
    //   assertThat(
    //   new Solution().constructFromPrePost(new int[] {1}, new int[] {1}).toString(),
    //   equalTo("1"));
    #[test]
    fn test_construct_from_pre_post2() {
        // TODO: 翻译 Java 测试
    }
}
