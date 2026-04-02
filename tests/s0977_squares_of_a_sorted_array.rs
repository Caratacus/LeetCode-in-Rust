// Tests for Problem 0977: Squares of a Sorted Array
// Java reference: src/test/java/g0901_1000/s0977_squares_of_a_sorted_array/SolutionTest.java

use leetcode_in_rust::s0977::squares_of_a_sorted_array::Solution;

#[test]
fn test_sorted_squares() {
    assert_eq!(
        Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
        vec![0, 1, 9, 16, 100]
    );
}

#[test]
fn test_sorted_squares2() {
    assert_eq!(
        Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
        vec![4, 9, 9, 49, 121]
    );
}
