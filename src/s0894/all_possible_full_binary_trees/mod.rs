// Problem 0894: all possible full binary trees

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        todo!()
    }

    pub fn helper(n: i32, dp: Vec<Vec<Option<Rc<RefCell<TreeNode>>>>>) -> () {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void allPossibleFBT()
    //   assertThat(
    //   new Solution().allPossibleFBT(7).toString(),
    //   equalTo(
    //   "[0,0,0,0,0,0,0, 0,0,0,0,0,0,0, 0,0,0,0,0,0,0, 0,0,0,0,0,0,0, 0,0,0,0,0,0,0]"));
    #[test]
    fn test_all_possible_fbt() {
        // TODO: 翻译 Java 测试
    }

    // Java: void allPossibleFBT2()
    //   assertThat(new Solution().allPossibleFBT(3).toString(), equalTo("[0,0,0]"));
    #[test]
    fn test_all_possible_fbt2() {
        // TODO: 翻译 Java 测试
    }
}
