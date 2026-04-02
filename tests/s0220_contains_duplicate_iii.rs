// Tests for Problem 0220: Contains Duplicate III
// Java reference: src/test/java/g0201_0300/s0220_contains_duplicate_iii/SolutionTest.java

use leetcode_in_rust::s0220::contains_duplicate_iii::Solution;

#[test]
fn test_contains_nearby_almost_duplicate() {
    assert_eq!(Solution::contains_nearby_almost_duplicate(vec![1, 2, 3, 1], 3, 0), true);
}

#[test]
fn test_contains_nearby_almost_duplicate2() {
    assert_eq!(Solution::contains_nearby_almost_duplicate(vec![1, 0, 1, 1], 1, 2), true);
}

#[test]
fn test_contains_nearby_almost_duplicate3() {
    assert_eq!(Solution::contains_nearby_almost_duplicate(vec![1, 5, 9, 1, 5, 9], 2, 3), false);
}
