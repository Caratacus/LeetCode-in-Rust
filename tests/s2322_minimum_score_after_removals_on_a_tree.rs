// Tests for Problem 2322: Minimum Score After Removals on a Tree
// Java reference: src/test/java/g2301_2400/s2322_minimum_score_after_removals_on_a_tree/SolutionTest.java

use leetcode_in_rust::s2322::minimum_score_after_removals_on_a_tree::Solution;

#[test]
fn test_minimum_score() {
    assert_eq!(
        Solution::minimum_score(vec![1, 5, 5, 4, 11], vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]]),
        9
    );
}

#[test]
fn test_minimum_score2() {
    assert_eq!(
        Solution::minimum_score(
            vec![5, 5, 2, 4, 4, 2],
            vec![vec![0, 1], vec![1, 2], vec![5, 2], vec![4, 3], vec![1, 3]]
        ),
        0
    );
}
