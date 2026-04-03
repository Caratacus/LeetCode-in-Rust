// Tests for Problem 1748: Sum of Unique Elements
// Java reference: src/test/java/g1701_1800/s1748_sum_of_unique_elements/SolutionTest.java

use leetcode_in_rust::s1748::sum_of_unique_elements::Solution;

#[test]
fn test_sum_of_unique() {
    assert_eq!(Solution::sum_of_unique(vec![1, 2, 3, 2]), 4);
}

#[test]
fn test_sum_of_unique2() {
    assert_eq!(Solution::sum_of_unique(vec![1, 1, 1, 1, 1]), 0);
}

#[test]
fn test_sum_of_unique3() {
    assert_eq!(Solution::sum_of_unique(vec![1, 2, 3, 4, 5]), 15);
}
