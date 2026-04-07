// Tests for Problem 1993: Operations on Tree
// Java reference: src/test/java/g1901_2000/s1993_operations_on_tree/SolutionTest.java

use leetcode_in_rust::s1993::operations_on_tree::LockingTree;

#[test]
fn test_locking_tree() {
    let mut locking_tree = LockingTree::new(vec![-1, 0, 0, 1, 1, 2, 2]);
    // return true because node 2 is unlocked.
    assert_eq!(locking_tree.lock(2, 2), true);
    // Node 2 will now be locked by user 2.
    // return false because user 3 cannot unlock a node locked by user 2.
    assert_eq!(locking_tree.unlock(2, 3), false);
    // return false because node 2 is already locked.
    assert_eq!(locking_tree.lock(4, 5), false);
    // return true because node 4 is unlocked.
    // Node 4 will now be locked by user 5.
    assert_eq!(locking_tree.lock(4, 5), true);
    // return true because node 0 is unlocked and has at least one locked descendant (2 and 4).
    // Node 0 will now be locked by user 1.
    assert_eq!(locking_tree.upgrade(0, 1), true);
    // return false because node 0 is already locked.
    assert_eq!(locking_tree.lock(0, 1), false);
}
