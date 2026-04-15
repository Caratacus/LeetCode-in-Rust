// Tests for Problem 3471: Find the Largest Almost Missing Integer
// Java reference: src/test/java/g3401_3500/s3471_find_the_largest_almost_missing_integer/SolutionTest.java

use leetcode_in_rust::s3471::find_the_largest_almost_missing_integer::Solution;

#[test]
fn test_largest_integer() {
    assert_eq!(Solution::largest_integer(vec![3, 9, 2, 1, 7], 3), 7);
}

#[test]
fn test_largest_integer2() {
    assert_eq!(Solution::largest_integer(vec![3, 9, 7, 2, 1, 7], 4), 3);
}

#[test]
fn test_largest_integer3() {
    assert_eq!(Solution::largest_integer(vec![0, 0], 1), -1);
}
