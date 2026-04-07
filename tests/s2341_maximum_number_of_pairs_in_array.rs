// Tests for Problem 2341: Maximum Number of Pairs in Array
// Java reference: src/test/java/g2301_2400/s2341_maximum_number_of_pairs_in_array/SolutionTest.java

use leetcode_in_rust::s2341::maximum_number_of_pairs_in_array::Solution;

#[test]
fn test_number_of_pairs() {
    assert_eq!(
        Solution::number_of_pairs(vec![1, 3, 2, 1, 3, 2, 2]),
        vec![3, 1]
    );
}

#[test]
fn test_number_of_pairs2() {
    assert_eq!(Solution::number_of_pairs(vec![1, 1]), vec![1, 0]);
}

#[test]
fn test_number_of_pairs3() {
    assert_eq!(Solution::number_of_pairs(vec![0]), vec![0, 1]);
}
