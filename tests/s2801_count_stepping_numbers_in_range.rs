// Tests for Problem 2801: Count Stepping Numbers in Range
// Java reference: src/test/java/g2701_2800/s2801_count_stepping_numbers_in_range/SolutionTest.java

use leetcode_in_rust::s2801::count_stepping_numbers_in_range::Solution;

#[test]
fn test_count_stepping_numbers() {
    assert_eq!(Solution::count_stepping_numbers("1".to_string(), "11".to_string()), 10);
}

#[test]
fn test_count_stepping_numbers2() {
    assert_eq!(Solution::count_stepping_numbers("90".to_string(), "101".to_string()), 2);
}
