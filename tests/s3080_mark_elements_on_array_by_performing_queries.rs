// Tests for Problem 3080: Mark Elements on Array by Performing Queries
// Java reference: src/test/java/g3001_3100/s3080_mark_elements_on_array_by_performing_queries/SolutionTest.java

use leetcode_in_rust::s3080::mark_elements_on_array_by_performing_queries::Solution;

#[test]
fn test_unmarked_sum_array() {
    assert_eq!(
        Solution::unmarked_sum_array(
            vec![1, 2, 2, 1, 2, 3, 1],
            vec![vec![1, 2], vec![3, 3], vec![4, 2]]
        ),
        vec![8, 3, 0]
    );
}

#[test]
fn test_unmarked_sum_array2() {
    assert_eq!(
        Solution::unmarked_sum_array(vec![1, 4, 2, 3], vec![vec![0, 1]]),
        vec![7]
    );
}
