// Tests for Problem 0910: Smallest Range II
// Java reference: src/test/java/g0901_1000/s0910_smallest_range_ii/SolutionTest.java

use leetcode_in_rust::s0910::smallest_range_ii::Solution;

#[test]
fn test_smallest_range_ii() {
    let result = Solution::smallest_range_ii(vec![1], 0);
    assert_eq!(result, 0);
}

#[test]
fn test_smallest_range_ii2() {
    let result = Solution::smallest_range_ii(vec![0, 10], 2);
    assert_eq!(result, 6);
}

#[test]
fn test_smallest_range_ii3() {
    let result = Solution::smallest_range_ii(vec![1, 3, 6], 3);
    assert_eq!(result, 3);
}
