// Tests for Problem 3388: Count Beautiful Splits in an Array
// Java reference: src/test/java/g3301_3400/s3388_count_beautiful_splits_in_an_array/SolutionTest.java

use leetcode_in_rust::s3388::count_beautiful_splits_in_an_array::Solution;

#[test]
fn test_beautiful_splits() {
    assert_eq!(Solution::beautiful_splits(vec![1, 1, 2, 1]), 2);
}

#[test]
fn test_beautiful_splits2() {
    assert_eq!(Solution::beautiful_splits(vec![1, 2, 3, 4]), 0);
}
