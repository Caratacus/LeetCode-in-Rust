// Tests for Problem 0973: K Closest Points to Origin
// Java reference: src/test/java/g0901_1000/s0973_k_closest_points_to_origin/SolutionTest.java

use leetcode_in_rust::s0973::k_closest_points_to_origin::Solution;

#[test]
fn test_k_closest() {
    let result = Solution::k_closest(vec![vec![1, 3], vec![-2, 2]], 1);
    assert_eq!(result, vec![vec![-2, 2]]);
}

#[test]
fn test_k_closest2() {
    let result = Solution::k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2);
    // Result should contain two closest points
    assert_eq!(result.len(), 2);
}
