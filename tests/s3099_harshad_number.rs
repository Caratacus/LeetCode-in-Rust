// Tests for Problem 3099: Harshad Number
// Java reference: src/test/java/g3001_3100/s3099_harshad_number/SolutionTest.java

use leetcode_in_rust::s3099::harshad_number::Solution;

#[test]
fn test_sum_of_the_digits_of_harshad_number() {
    assert_eq!(Solution::sum_of_the_digits_of_harshad_number(18), 9);
}

#[test]
fn test_sum_of_the_digits_of_harshad_number2() {
    assert_eq!(Solution::sum_of_the_digits_of_harshad_number(23), -1);
}
