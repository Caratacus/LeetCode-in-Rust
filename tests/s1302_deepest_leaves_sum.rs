// Tests for Problem 1302: Deepest Leaves Sum
// Java reference: src/test/java/g1301_1400/s1302_deepest_leaves_sum/SolutionTest.java

use leetcode_in_rust::s1302::deepest_leaves_sum::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_deepest_leaves_sum() {
    let root = tree_from_vec(vec![
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        None,
        Some(6),
        Some(7),
        None,
        None,
        None,
        None,
        Some(8),
    ]);
    assert_eq!(Solution::deepest_leaves_sum(root), 15);
}

#[test]
fn test_deepest_leaves_sum2() {
    let root = tree_from_vec(vec![
        Some(6),
        Some(7),
        Some(8),
        Some(2),
        Some(7),
        Some(1),
        Some(3),
        Some(9),
        None,
        Some(1),
        Some(4),
        None,
        None,
        None,
        Some(5),
    ]);
    assert_eq!(Solution::deepest_leaves_sum(root), 19);
}
