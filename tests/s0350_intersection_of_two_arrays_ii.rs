// Tests for Problem 0350: Intersection of Two Arrays II
// Java reference: src/test/java/g0301_0400/s0350_intersection_of_two_arrays_ii/SolutionTest.java

use leetcode_in_rust::s0350::intersection_of_two_arrays_ii::Solution;

#[test]
fn test_intersect() {
    let mut result = Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]);
    result.sort();
    assert_eq!(result, vec![2, 2]);
}

#[test]
fn test_intersect2() {
    let mut result = Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
    result.sort();
    // Result can be [4, 9] or [9, 4] depending on implementation
    assert!(result == vec![4, 9] || result == vec![9, 4]);
}
