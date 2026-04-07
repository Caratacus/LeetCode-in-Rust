// Tests for Problem 2216: Minimum Deletions to Make Array Beautiful
// Java reference: src/test/java/g2201_2300/s2216_minimum_deletions_to_make_array_beautiful/SolutionTest.java

use leetcode_in_rust::s2216::minimum_deletions_to_make_array_beautiful::Solution;

#[test]
fn test_min_deletion() {
    assert_eq!(Solution::min_deletion(vec![1, 1, 2, 3, 5]), 1);
}

#[test]
fn test_min_deletion2() {
    assert_eq!(Solution::min_deletion(vec![1, 1, 2, 2, 3, 3]), 2);
}
