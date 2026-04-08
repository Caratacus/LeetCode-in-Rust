// Tests for Problem 3129: Find All Possible Stable Binary Arrays I
// Java reference: src/test/java/g3101_3200/s3129_find_all_possible_stable_binary_arrays_i/SolutionTest.java

use leetcode_in_rust::s3129::find_all_possible_stable_binary_arrays_i::Solution;

#[test]
fn test_number_of_stable_arrays() {
    assert_eq!(Solution::number_of_stable_arrays(1, 1, 2), 2);
}

#[test]
fn test_number_of_stable_arrays2() {
    assert_eq!(Solution::number_of_stable_arrays(1, 2, 1), 1);
}

