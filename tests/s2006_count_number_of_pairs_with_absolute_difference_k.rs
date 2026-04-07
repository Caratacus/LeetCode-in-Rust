// Tests for Problem 2006: Count Number of Pairs With Absolute Difference K
// Java reference: src/test/java/g2001_2100/s2006_count_number_of_pairs_with_absolute_difference_k/SolutionTest.java

use leetcode_in_rust::s2006::count_number_of_pairs_with_absolute_difference_k::Solution;

#[test]
fn test_count_k_difference() {
    assert_eq!(Solution::count_k_difference(vec![1, 2, 2, 1], 1), 4);
}

#[test]
fn test_count_k_difference2() {
    assert_eq!(Solution::count_k_difference(vec![1, 3], 3), 0);
}

#[test]
fn test_count_k_difference3() {
    assert_eq!(Solution::count_k_difference(vec![3, 2, 1, 5, 4], 2), 3);
}
