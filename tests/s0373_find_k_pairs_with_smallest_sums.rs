// Tests for Problem 0373: Find K Pairs with Smallest Sums
// Java reference: src/test/java/g0301_0400/s0373_find_k_pairs_with_smallest_sums/SolutionTest.java

use leetcode_in_rust::s0373::find_k_pairs_with_smallest_sums::Solution;

#[test]
fn test_k_smallest_pairs() {
    let result = Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3);
    assert_eq!(result.len(), 3);
    // The result should contain [1,2], [1,4], [1,6] in some order
}

#[test]
fn test_k_smallest_pairs2() {
    let result = Solution::k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 2);
    assert_eq!(result.len(), 2);
    // The result should contain [1,1], [1,1] in some order
}

#[test]
fn test_k_smallest_pairs3() {
    let result = Solution::k_smallest_pairs(vec![1, 2], vec![3], 3);
    assert_eq!(result.len(), 2);
    // The result should contain [1,3], [2,3] in some order
}
