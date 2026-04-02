// Tests for Problem 0368: Largest Divisible Subset
// Java reference: src/test/java/g0301_0400/s0368_largest_divisible_subset/SolutionTest.java

use leetcode_in_rust::s0368::largest_divisible_subset::Solution;

#[test]
fn test_largest_divisible_subset() {
    let mut result = Solution::largest_divisible_subset(vec![1, 2, 3]);
    result.sort();
    // Valid answers: [1, 2] or [1, 3]
    assert!(result == vec![1, 2] || result == vec![1, 3]);
}

#[test]
fn test_largest_divisible_subset2() {
    let mut result = Solution::largest_divisible_subset(vec![1, 2, 4, 8]);
    result.sort();
    assert_eq!(result, vec![1, 2, 4, 8]);
}
