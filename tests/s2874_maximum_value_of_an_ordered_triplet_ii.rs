// Tests for Problem 2874: Maximum Value of an Ordered Triplet II
// Java reference: src/test/java/g2801_2900/s2874_maximum_value_of_an_ordered_triplet_ii/SolutionTest.java

use leetcode_in_rust::s2874::maximum_value_of_an_ordered_triplet_ii::Solution;

#[test]
fn test_maximum_triplet_value() {
    assert_eq!(Solution::maximum_triplet_value(vec![12, 6, 1, 2, 7]), 77);
}

#[test]
fn test_maximum_triplet_value2() {
    assert_eq!(Solution::maximum_triplet_value(vec![1, 10, 3, 4, 19]), 133);
}

#[test]
fn test_maximum_triplet_value3() {
    assert_eq!(Solution::maximum_triplet_value(vec![1, 2, 3]), 0);
}
