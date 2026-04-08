// Tests for Problem 3161: Block Placement Queries
// Java reference: src/test/java/g3101_3200/s3161_block_placement_queries/SolutionTest.java
use leetcode_in_rust::s3161::block_placement_queries::Solution;
#[test]
fn test_get_results() {
    assert_eq!(
        Solution::get_results(vec![vec![1, 2], vec![2, 3, 3], vec![2, 3, 1], vec![2, 2, 2]]),
        vec![false, true, true]
    );
}
#[test]
fn test_get_results2() {
    assert_eq!(
        Solution::get_results(
            vec![vec![1, 7], vec![2, 7, 6], vec![1, 2], vec![2, 7, 5], vec![2, 7, 6]]
        ),
        vec![true, true, false]
    );
}
