// Tests for Problem 1342: Number of Steps to Reduce a Number to Zero
// Java reference: src/test/java/g1301_1400/s1342_number_of_steps_to_reduce_a_number_to_zero/SolutionTest.java

use leetcode_in_rust::s1342::number_of_steps_to_reduce_a_number_to_zero::Solution;

#[test]
fn test_number_of_steps() {
    assert_eq!(Solution::number_of_steps(14), 6);
}

#[test]
fn test_number_of_steps2() {
    assert_eq!(Solution::number_of_steps(8), 4);
}
