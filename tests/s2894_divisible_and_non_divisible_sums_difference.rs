// Tests for Problem 2894: Divisible and Non-divisible Sums Difference
// Java reference: src/test/java/g2801_2900/s2894_divisible_and_non_divisible_sums_difference/SolutionTest.java

use leetcode_in_rust::s2894::divisible_and_non_divisible_sums_difference::Solution;

#[test]
fn test_difference_of_sums() {
    assert_eq!(Solution::difference_of_sums(10, 3), 19);
}

#[test]
fn test_difference_of_sums2() {
    assert_eq!(Solution::difference_of_sums(5, 6), 15);
}

#[test]
fn test_difference_of_sums3() {
    assert_eq!(Solution::difference_of_sums(5, 1), -15);
}
