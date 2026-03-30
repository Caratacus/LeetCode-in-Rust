// Problem 1483: kth ancestor of a tree node

pub struct TreeAncestor {
    n: i32,
    parent: Vec<i32>,
}

impl TreeAncestor {
    pub fn new(n: i32, parent: Vec<i32>) -> Self {
        todo!()
    }

    pub fn get_kth_ancestor(&self, node: i32, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void treeAncestor()
    //   TreeAncestor treeAncestor = new TreeAncestor(7, new int[] {-1, 0, 0, 1, 1, 2, 2});
    //   assertThat(treeAncestor.getKthAncestor(3, 1), equalTo(1));
    //   assertThat(treeAncestor.getKthAncestor(5, 2), equalTo(0));
    //   assertThat(treeAncestor.getKthAncestor(6, 3), equalTo(-1));
    #[test]
    fn test_tree_ancestor() {
        // TODO: 翻译 Java 测试
    }

    // Java: void treeAncestor2()
    //   TreeAncestor treeAncestor =
    //   new TreeAncestor(
    //   21,
    //   new int[] {
    //   -1, 0, 0, 1, 1, 2, 2, -1, 0, 0, 1, 1, 2, 2, -1, 0, 0, 1, 1, 2, 2
    //   ... (4 more lines)
    #[test]
    fn test_tree_ancestor2() {
        // TODO: 翻译 Java 测试
    }
}
