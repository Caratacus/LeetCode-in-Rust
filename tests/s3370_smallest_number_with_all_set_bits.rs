// Tests for Problem 3370: Smallest Number With All Set Bits
// Java reference: src/test/java/g3301_3400/s3370_smallest_number_with_all_set_bits/SolutionTest.java

use leetcode_in_rust::s3370::smallest_number_with_all_set_bits::Solution;

#[test]
fn test_smallest_number() {
    assert_eq!(Solution::smallest_number(5), 7);
}

#[test]
fn test_smallest_number2() {
    assert_eq!(Solution::smallest_number(10), 15);
}

#[test]
fn test_smallest_number3() {
    assert_eq!(Solution::smallest_number(3), 3);
}
