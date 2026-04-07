// Tests for Problem 2601: Prime Subtraction Operation
// Java reference: src/test/java/g2601_2700/s2601_prime_subtraction_operation/SolutionTest.java

use leetcode_in_rust::s2601::prime_subtraction_operation::Solution;

#[test]
fn test_prime_sub_operation() {
    assert_eq!(Solution::prime_sub_operation(vec![4, 9, 6, 10]), true);
}

#[test]
fn test_prime_sub_operation2() {
    assert_eq!(Solution::prime_sub_operation(vec![6, 8, 11, 12]), true);
}
