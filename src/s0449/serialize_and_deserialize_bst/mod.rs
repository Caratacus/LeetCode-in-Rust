// Problem 0449: serialize and deserialize bst

use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Codec {
    // TODO: 添加字段
}

impl Codec {
    pub fn serialize(&mut self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        todo!()
    }

    pub fn deserialize(&mut self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void serializeDeserialize()
    //   TreeNode expectedRoot = new TreeNode(3);
    //   expectedRoot.left = new TreeNode(1);
    //   expectedRoot.right = new TreeNode(4);
    //   expectedRoot.left.right = new TreeNode(2);
    //   assertThat(
    //   ... (11 more lines)
    #[test]
    fn test_serialize_deserialize() {
        // TODO: 翻译 Java 测试
    }
}
