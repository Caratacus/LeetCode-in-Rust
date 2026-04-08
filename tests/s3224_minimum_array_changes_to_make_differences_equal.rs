// Tests for Problem 3224: Minimum Array Changes to Make Differences Equal
// Java reference: src/test/java/g3201_3300/s3224_minimum_array_changes_to_make_differences_equal/SolutionTest.java

use leetcode_in_rust::s3224::minimum_array_changes_to_make_differences_equal::Solution;

#[test]
fn test_min_changes() {
    assert_eq!(Solution::min_changes(vec![1, 0, 1, 2, 4, 3], 4), 2);
}

#[test]
fn test_min_changes2() {
    assert_eq!(Solution::min_changes(vec![18, 10], 0), 0);
}

#[test]
fn test_min_changes3() {
    assert_eq!(Solution::min_changes(vec![1, 1, 1, 1], 0), 2);
}
