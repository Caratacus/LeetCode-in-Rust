// Problem 1993: operations on tree

pub struct LockingTree {
    parent: Vec<i32>,
}

impl LockingTree {
    pub fn new(parent: Vec<i32>) -> Self {
        todo!()
    }

    pub fn lock(&mut self, id: i32, user: i32) -> bool {
        todo!()
    }

    pub fn unlock(&mut self, id: i32, user: i32) -> bool {
        todo!()
    }

    pub fn upgrade(&mut self, id: i32, user: i32) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void lockingTree()
    //   LockingTree lockingTree = new LockingTree(new int[] {-1, 0, 0, 1, 1, 2, 2});
    //   // return true because node 2 is unlocked.
    //   assertThat(lockingTree.lock(2, 2), equalTo(true));
    //   // Node 2 will now be locked by user 2.
    //   // return false because user 3 cannot unlock a node locked by user 2.
    //   ... (11 more lines)
    #[test]
    fn test_locking_tree() {
        // TODO: 翻译 Java 测试
    }
}
