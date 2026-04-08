// Tests for Problem 3049: Earliest Second to Mark Indices II
// Java reference: src/test/java/g3001_3100/s3049_earliest_second_to_mark_indices_ii/SolutionTest.java

use leetcode_in_rust::s3049::earliest_second_to_mark_indices_ii::Solution;

#[test]
fn test_earliest_second_to_mark_indices() {
    assert_eq!(
        Solution::earliest_second_to_mark_indices(vec![3, 2, 3], vec![1, 3, 2, 2, 2, 2, 3]),
        6
    );
}

#[test]
fn test_earliest_second_to_mark_indices2() {
    assert_eq!(
        Solution::earliest_second_to_mark_indices(vec![0, 0, 1, 2], vec![1, 2, 1, 2, 1, 2, 1, 2]),
        7
    );
}

#[test]
fn test_earliest_second_to_mark_indices3() {
    assert_eq!(
        Solution::earliest_second_to_mark_indices(vec![1, 2, 3], vec![1, 2, 3]),
        -1
    );
}
