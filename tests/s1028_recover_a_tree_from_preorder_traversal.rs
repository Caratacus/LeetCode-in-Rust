// Tests for Problem 1028: Recover a Tree From Preorder Traversal
// Java reference: src/test/java/g1001_1100/s1028_recover_a_tree_from_preorder_traversal/SolutionTest.java

use leetcode_in_rust::s1028::recover_a_tree_from_preorder_traversal::Solution;
use leetcode_in_rust::utils::tree_utils::{tree_from_vec, tree_to_vec};

#[test]
fn test_recover_from_preorder() {
    let result = Solution::recover_from_preorder("1-2--3--4-5--6--7".to_string());
    let expected = tree_from_vec(vec![Some(1), Some(2), Some(5), Some(3), Some(4), Some(6), Some(7)]);
    assert_eq!(tree_to_vec(result), tree_to_vec(expected));
}

#[test]
fn test_recover_from_preorder2() {
    let result = Solution::recover_from_preorder("1-2--3---4-5--6---7".to_string());
    let expected = tree_from_vec(vec![Some(1), Some(2), Some(5), Some(3), None, Some(6), None, Some(4), None, Some(7)]);
    assert_eq!(tree_to_vec(result), tree_to_vec(expected));
}

#[test]
fn test_recover_from_preorder3() {
    let result = Solution::recover_from_preorder("1-401--349---90--88".to_string());
    let expected = tree_from_vec(vec![Some(1), Some(401), None, Some(349), Some(88), Some(90)]);
    assert_eq!(tree_to_vec(result), tree_to_vec(expected));
}
