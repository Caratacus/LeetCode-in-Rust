// Tests for Problem 3130: Find All Possible Stable Binary Arrays II
// Java reference: src/test/java/g3101_3200/s3130_find_all_possible_stable_binary_arrays_ii/SolutionTest.java

use leetcode_in_rust::s3130::find_all_possible_stable_binary_arrays_ii::Solution;

#[test]
fn test_number_of_stable_arrays() {
    assert_eq!(Solution::number_of_stable_arrays(1, 1, 2), 2);
}

#[test]
fn test_number_of_stable_arrays2() {
    assert_eq!(Solution::number_of_stable_arrays(1, 2, 1), 1);
}

#[test]
fn test_number_of_stable_arrays3() {
    assert_eq!(Solution::number_of_stable_arrays(3, 3, 2), 14);
}
