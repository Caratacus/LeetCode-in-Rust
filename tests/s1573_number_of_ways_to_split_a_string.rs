// Tests for Problem 1573: Number of Ways to Split a String
// Java reference: src/test/java/g1501_1600/s1573_number_of_ways_to_split_a_string/SolutionTest.java

use leetcode_in_rust::s1573::number_of_ways_to_split_a_string::Solution;

#[test]
fn test_num_ways() {
    assert_eq!(Solution::num_ways("10101".to_string()), 4);
}

#[test]
fn test_num_ways2() {
    assert_eq!(Solution::num_ways("1001".to_string()), 0);
}

#[test]
fn test_num_ways3() {
    assert_eq!(Solution::num_ways("0000".to_string()), 3);
}
