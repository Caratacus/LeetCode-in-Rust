// Tests for Problem 3046: Split the Array
// Java reference: src/test/java/g3001_3100/s3046_split_the_array/SolutionTest.java

use leetcode_in_rust::s3046::split_the_array::Solution;

#[test]
fn test_is_possible_to_split() {
    assert_eq!(Solution::is_possible_to_split(vec![1, 1, 2, 2, 3, 4]), true);
}

#[test]
fn test_is_possible_to_split2() {
    assert_eq!(Solution::is_possible_to_split(vec![1, 1, 1, 1]), false);
}
