// Tests for Problem 1420: Build Array Where You Can Find the Maximum Exactly K Comparisons
// Java reference: src/test/java/g1401_1500/s1420_build_array_where_you_can_find_the_maximum_exactly_k_comparisons/SolutionTest.java

use leetcode_in_rust::s1420::build_array_where_you_can_find_the_maximum_exactly_k_comparisons::Solution;

#[test]
fn test_num_of_arrays() {
    assert_eq!(Solution::num_of_arrays(2, 3, 1), 6);
}

#[test]
fn test_num_of_arrays2() {
    assert_eq!(Solution::num_of_arrays(5, 2, 3), 0);
}

#[test]
fn test_num_of_arrays3() {
    assert_eq!(Solution::num_of_arrays(9, 1, 1), 1);
}
