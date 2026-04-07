// Tests for Problem 2729: Check if The Number is Fascinating
// Java reference: src/test/java/g2701_2800/s2729_check_if_the_number_is_fascinating/SolutionTest.java

use leetcode_in_rust::s2729::check_if_the_number_is_fascinating::Solution;

#[test]
fn test_is_fascinating() {
    assert_eq!(Solution::is_fascinating(192), true);
}

#[test]
fn test_is_fascinating2() {
    assert_eq!(Solution::is_fascinating(100), false);
}
