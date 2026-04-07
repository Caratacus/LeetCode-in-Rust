// Tests for Problem 2553: Separate the Digits in an Array
// Java reference: src/test/java/g2501_2600/s2553_separate_the_digits_in_an_array/SolutionTest.java

use leetcode_in_rust::s2553::separate_the_digits_in_an_array::Solution;

#[test]
fn test_separate_digits() {
    assert_eq!(
        Solution::separate_digits(vec![13, 25, 83, 77]),
        vec![1, 3, 2, 5, 8, 3, 7, 7]
    );
}

#[test]
fn test_separate_digits2() {
    assert_eq!(
        Solution::separate_digits(vec![7, 1, 3, 9]),
        vec![7, 1, 3, 9]
    );
}
