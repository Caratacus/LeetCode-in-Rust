// Tests for Problem 1525: Number of Good Ways to Split a String
// Java reference: src/test/java/g1501_1600/s1525_number_of_good_ways_to_split_a_string/SolutionTest.java

use leetcode_in_rust::s1525::number_of_good_ways_to_split_a_string::Solution;

#[test]
fn test_num_splits() {
    assert_eq!(Solution::num_splits("aacaba".to_string()), 2);
}

#[test]
fn test_num_splits2() {
    assert_eq!(Solution::num_splits("abcd".to_string()), 1);
}
