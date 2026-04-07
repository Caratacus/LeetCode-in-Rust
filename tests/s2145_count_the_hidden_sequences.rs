// Tests for Problem 2145: Count the Hidden Sequences
// Java reference: src/test/java/g2101_2200/s2145_count_the_hidden_sequences/SolutionTest.java

use leetcode_in_rust::s2145::count_the_hidden_sequences::Solution;

#[test]
fn test_number_of_arrays() {
    assert_eq!(Solution::number_of_arrays(vec![1, -3, 4], 1, 6), 2);
}

#[test]
fn test_number_of_arrays2() {
    assert_eq!(Solution::number_of_arrays(vec![3, -4, 5, 1, -2], -4, 5), 4);
}

#[test]
fn test_number_of_arrays3() {
    assert_eq!(Solution::number_of_arrays(vec![4, -7, 2], 3, 6), 0);
}

#[test]
fn test_number_of_arrays4() {
    assert_eq!(Solution::number_of_arrays(vec![4, -7, 2], 3, 3), 0);
}
