// Tests for Problem 2709: Greatest Common Divisor Traversal
// Java reference: src/test/java/g2701_2800/s2709_greatest_common_divisor_traversal/SolutionTest.java

use leetcode_in_rust::s2709::greatest_common_divisor_traversal::Solution;

#[test]
fn test_can_traverse_all_pairs() {
    assert_eq!(Solution::can_traverse_all_pairs(vec![2, 3, 6]), true);
}

#[test]
fn test_can_traverse_all_pairs2() {
    assert_eq!(Solution::can_traverse_all_pairs(vec![3, 9, 5]), false);
}

#[test]
fn test_can_traverse_all_pairs3() {
    assert_eq!(Solution::can_traverse_all_pairs(vec![4, 3, 12, 8]), true);
}
