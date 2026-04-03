// Tests for Problem 1296: Divide Array in Sets of K Consecutive Numbers
// Java reference: src/test/java/g1201_1300/s1296_divide_array_in_sets_of_k_consecutive_numbers/SolutionTest.java

use leetcode_in_rust::s1296::divide_array_in_sets_of_k_consecutive_numbers::Solution;

#[test]
fn test_is_possible_divide() {
    assert_eq!(Solution::is_possible_divide(vec![1, 2, 3, 3, 4, 4, 5, 6], 4), true);
}

#[test]
fn test_is_possible_divide2() {
    assert_eq!(Solution::is_possible_divide(vec![3, 2, 1, 2, 3, 4, 7, 8], 3), true);
}

#[test]
fn test_is_possible_divide3() {
    assert_eq!(Solution::is_possible_divide(vec![1, 2, 3, 4], 3), false);
}
