// Tests for Problem 0941: Valid Mountain Array
// Java reference: src/test/java/g0901_1000/s0941_valid_mountain_array/SolutionTest.java

use leetcode_in_rust::s0941::valid_mountain_array::Solution;

#[test]
fn test_valid_mountain_array() {
    assert_eq!(Solution::valid_mountain_array(vec![2, 1]), false);
}

#[test]
fn test_valid_mountain_array2() {
    assert_eq!(Solution::valid_mountain_array(vec![3, 5, 5]), false);
}

#[test]
fn test_valid_mountain_array3() {
    assert_eq!(Solution::valid_mountain_array(vec![0, 3, 2, 1]), true);
}
