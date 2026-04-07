// Tests for Problem 1947: Maximum Compatibility Score Sum
// Java reference: src/test/java/g1901_2000/s1947_maximum_compatibility_score_sum/SolutionTest.java

use leetcode_in_rust::s1947::maximum_compatibility_score_sum::Solution;

#[test]
fn test_max_compatibility_sum() {
    assert_eq!(
        Solution::max_compatibility_sum(
            vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 1]],
            vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 1, 0]]
        ),
        8
    );
}

#[test]
fn test_max_compatibility_sum2() {
    assert_eq!(
        Solution::max_compatibility_sum(
            vec![vec![0, 0], vec![0, 0], vec![0, 0]],
            vec![vec![1, 1], vec![1, 1], vec![1, 1]]
        ),
        0
    );
}
