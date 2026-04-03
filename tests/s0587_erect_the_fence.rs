// Tests for Problem 0587: Erect the Fence
// Java reference: src/test/java/g0501_0600/s0587_erect_the_fence/SolutionTest.java

use leetcode_in_rust::s0587::erect_the_fence::Solution;

#[test]
fn test_outer_trees() {
    let result = Solution::outer_trees(vec![
        vec![1, 1], vec![2, 2], vec![2, 0], vec![2, 4], vec![3, 3], vec![4, 2]
    ]);
    let expected = vec![vec![1, 1], vec![2, 0], vec![4, 2], vec![3, 3], vec![2, 4]];
    // Order may vary, check all expected points are in result
    for point in &expected {
        assert!(result.contains(point));
    }
    assert_eq!(result.len(), expected.len());
}

#[test]
fn test_outer_trees2() {
    let result = Solution::outer_trees(vec![vec![1, 2], vec![2, 2], vec![4, 2]]);
    let expected = vec![vec![1, 2], vec![2, 2], vec![4, 2]];
    for point in &expected {
        assert!(result.contains(point));
    }
    assert_eq!(result.len(), expected.len());
}
