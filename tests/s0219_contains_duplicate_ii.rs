// Tests for Problem 0219: Contains Duplicate II
// Java reference: src/test/java/g0201_0300/s0219_contains_duplicate_ii/SolutionTest.java

use leetcode_in_rust::s0219::contains_duplicate_ii::Solution;

#[test]
fn test_contains_nearby_duplicate() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3), true);
}

#[test]
fn test_contains_nearby_duplicate2() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1), true);
}

#[test]
fn test_contains_nearby_duplicate3() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2), false);
}
