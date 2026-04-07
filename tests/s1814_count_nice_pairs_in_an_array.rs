// Tests for Problem 1814: Count Nice Pairs in an Array
// Java reference: src/test/java/g1801_1900/s1814_count_nice_pairs_in_an_array/SolutionTest.java

use leetcode_in_rust::s1814::count_nice_pairs_in_an_array::Solution;

#[test]
fn test_count_nice_pairs() {
    assert_eq!(Solution::count_nice_pairs(vec![42, 11, 1, 97]), 2);
}

#[test]
fn test_count_nice_pairs2() {
    assert_eq!(Solution::count_nice_pairs(vec![13, 10, 35, 24, 76]), 4);
}
