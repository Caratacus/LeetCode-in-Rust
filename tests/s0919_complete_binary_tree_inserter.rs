// Tests for Problem 0919: Complete Binary Tree Inserter
// Java reference: src/test/java/g0901_1000/s0919_complete_binary_tree_inserter/SolutionTest.java

use leetcode_in_rust::s0919::complete_binary_tree_inserter::CBTInserter;
use leetcode_in_rust::utils::tree_utils::{tree_from_vec, tree_to_vec};

#[test]
fn test_cbt_inserter_test() {
    let root = tree_from_vec(vec![Some(1), Some(2)]);
    let mut inserter = CBTInserter::new(root);
    assert_eq!(inserter.insert(3), 1);
    assert_eq!(inserter.insert(4), 2);
    let expected = tree_to_vec(inserter.get_root());
    assert_eq!(expected, vec![Some(1), Some(2), Some(3), Some(4)]);
}
