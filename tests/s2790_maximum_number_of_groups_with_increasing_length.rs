// Tests for Problem 2790: Maximum Number of Groups With Increasing Length
// Java reference: src/test/java/g2701_2800/s2790_maximum_number_of_groups_with_increasing_length/SolutionTest.java

use leetcode_in_rust::s2790::maximum_number_of_groups_with_increasing_length::Solution;

#[test]
fn test_max_increasing_groups() {
    assert_eq!(Solution::max_increasing_groups(vec![1, 2, 5]), 3);
}

#[test]
fn test_max_increasing_groups2() {
    assert_eq!(Solution::max_increasing_groups(vec![2, 1, 2]), 2);
}
