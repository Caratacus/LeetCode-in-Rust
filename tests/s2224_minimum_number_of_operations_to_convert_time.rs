// Tests for Problem 2224: Minimum Number of Operations to Convert Time
// Java reference: src/test/java/g2201_2300/s2224_minimum_number_of_operations_to_convert_time/SolutionTest.java

use leetcode_in_rust::s2224::minimum_number_of_operations_to_convert_time::Solution;

#[test]
fn test_convert_time() {
    assert_eq!(Solution::convert_time("02:30".to_string(), "04:35".to_string()), 3);
}

#[test]
fn test_convert_time2() {
    assert_eq!(Solution::convert_time("11:00".to_string(), "11:01".to_string()), 1);
}
