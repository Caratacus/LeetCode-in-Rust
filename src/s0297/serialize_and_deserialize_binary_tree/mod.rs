// Problem 0297: serialize and deserialize binary tree

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

    // Java: void codec()
    //   TreeNode treeNode =
    //   new TreeNode(1, new TreeNode(2), new TreeNode(3, new TreeNode(4), new TreeNode(5)));
    //   Codec codec = new Codec();
    //   String actual = codec.serialize(treeNode);
    //   assertThat(actual, equalTo("3e93ea**3eb3ec**3ed**"));
    //   ... (2 more lines)
    #[test]
    fn test_codec() {
        // TODO: 翻译 Java 测试
    }
}
