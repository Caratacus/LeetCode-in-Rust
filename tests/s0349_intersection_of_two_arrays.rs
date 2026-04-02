// Tests for Problem 0349: Intersection of Two Arrays
// Java reference: src/test/java/g0301_0400/s0349_intersection_of_two_arrays/SolutionTest.java

use leetcode_in_rust::s0349::intersection_of_two_arrays::Solution;

#[test]
fn test_intersection() {
    let mut result = Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]);
    result.sort();
    assert_eq!(result, vec![2]);
}

#[test]
fn test_intersection2() {
    let mut result = Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
    result.sort();
    assert_eq!(result, vec![4, 9]);
}
