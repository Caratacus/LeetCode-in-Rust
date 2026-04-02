// Tests for Problem 0938: Range Sum of BST
// Java reference: src/test/java/g0901_1000/s0938_range_sum_of_bst/SolutionTest.java

use leetcode_in_rust::s0938::range_sum_of_bst::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_range_sum_bst() {
    let root = tree_from_vec(vec![
        Some(10),
        Some(5),
        Some(15),
        Some(3),
        Some(7),
        None,
        Some(18),
    ]);
    assert_eq!(Solution::range_sum_bst(root, 7, 15), 32);
}

#[test]
fn test_range_sum_bst2() {
    let root = tree_from_vec(vec![
        Some(10),
        Some(5),
        Some(15),
        Some(3),
        Some(7),
        Some(13),
        Some(18),
        Some(1),
        None,
        Some(6),
    ]);
    assert_eq!(Solution::range_sum_bst(root, 6, 10), 23);
}
