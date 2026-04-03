// Tests for Problem 1629: Slowest Key
// Java reference: src/test/java/g1601_1700/s1629_slowest_key/SolutionTest.java

use leetcode_in_rust::s1629::slowest_key::Solution;

#[test]
fn test_slowest_key() {
    assert_eq!(Solution::slowest_key(vec![9, 29, 49, 50], "cbcd".to_string()), 'c');
}

#[test]
fn test_slowest_key2() {
    assert_eq!(Solution::slowest_key(vec![12, 23, 36, 46, 62], "spuda".to_string()), 'a');
}
