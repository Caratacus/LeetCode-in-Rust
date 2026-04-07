// Tests for Problem 2449: Minimum Number of Operations to Make Arrays Similar
// Java reference: src/test/java/g2401_2500/s2449_minimum_number_of_operations_to_make_arrays_similar/SolutionTest.java

use leetcode_in_rust::s2449::minimum_number_of_operations_to_make_arrays_similar::Solution;

#[test]
fn test_make_similar() {
    assert_eq!(
        Solution::make_similar(vec![8, 12, 6], vec![2, 14, 10]),
        2
    );
}

#[test]
fn test_make_similar2() {
    assert_eq!(Solution::make_similar(vec![1, 2, 5], vec![4, 1, 3]), 1);
}

#[test]
fn test_make_similar3() {
    assert_eq!(
        Solution::make_similar(vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1]),
        0
    );
}
