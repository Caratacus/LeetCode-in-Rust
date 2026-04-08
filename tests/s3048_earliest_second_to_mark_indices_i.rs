// Tests for Problem 3048: Earliest Second to Mark Indices I
// Java reference: src/test/java/g3001_3100/s3048_earliest_second_to_mark_indices_i/SolutionTest.java

use leetcode_in_rust::s3048::earliest_second_to_mark_indices_i::Solution;

#[test]
fn test_earliest_second_to_mark_indices() {
    assert_eq!(
        Solution::earliest_second_to_mark_indices(vec![2, 2, 0], vec![2, 2, 2, 2, 3, 2, 2, 1]),
        8
    );
}

#[test]
fn test_earliest_second_to_mark_indices2() {
    assert_eq!(
        Solution::earliest_second_to_mark_indices(vec![1, 3], vec![1, 1, 1, 2, 1, 1, 1]),
        6
    );
}

#[test]
fn test_earliest_second_to_mark_indices3() {
    assert_eq!(
        Solution::earliest_second_to_mark_indices(vec![0, 1], vec![2, 2, 2]),
        -1
    );
}
