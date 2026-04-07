// Tests for Problem 2496: Maximum Value of a String in an Array
// Java reference: src/test/java/g2401_2500/s2496_maximum_value_of_a_string_in_an_array/SolutionTest.java

use leetcode_in_rust::s2496::maximum_value_of_a_string_in_an_array::Solution;

#[test]
fn test_maximum_value() {
    assert_eq!(
        Solution::maximum_value(vec!["alic3".to_string(), "bob".to_string(), "3".to_string(), "4".to_string(), "00000".to_string()]),
        5
    );
}

#[test]
fn test_maximum_value2() {
    assert_eq!(
        Solution::maximum_value(vec!["1".to_string(), "01".to_string(), "001".to_string(), "0001".to_string()]),
        1
    );
}
