// Tests for Problem 2264: Largest 3-Same-Digit Number in String
// Java reference: src/test/java/g2201_2300/s2264_largest_3_same_digit_number_in_string/SolutionTest.java

use leetcode_in_rust::s2264::largest_3_same_digit_number_in_string::Solution;

#[test]
fn test_largest_good_integer() {
    assert_eq!(Solution::largest_good_integer("6777133339".to_string()), "777");
}

#[test]
fn test_largest_good_integer2() {
    assert_eq!(Solution::largest_good_integer("2300019".to_string()), "000");
}

#[test]
fn test_largest_good_integer3() {
    assert_eq!(Solution::largest_good_integer("42352338".to_string()), "");
}
