// Tests for Problem 3459: Length of Longest V Shaped Diagonal Segment
// Java reference: src/test/java/g3401_3500/s3459_length_of_longest_v_shaped_diagonal_segment/SolutionTest.java

use leetcode_in_rust::s3459::length_of_longest_v_shaped_diagonal_segment::Solution;

#[test]
fn test_len_of_v_diagonal() {
    assert_eq!(
        Solution::len_of_v_diagonal(vec![
            vec![2, 2, 1, 2, 2],
            vec![2, 0, 2, 2, 0],
            vec![2, 0, 1, 1, 0],
            vec![1, 0, 2, 2, 2],
            vec![2, 0, 0, 2, 2]
        ]),
        5
    );
}

#[test]
fn test_len_of_v_diagonal2() {
    assert_eq!(
        Solution::len_of_v_diagonal(vec![
            vec![2, 2, 2, 2, 2],
            vec![2, 0, 2, 2, 0],
            vec![2, 0, 1, 1, 0],
            vec![1, 0, 2, 2, 2],
            vec![2, 0, 0, 2, 2]
        ]),
        4
    );
}

#[test]
fn test_len_of_v_diagonal3() {
    assert_eq!(
        Solution::len_of_v_diagonal(vec![
            vec![1, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 0],
            vec![2, 0, 0, 0, 0],
            vec![0, 0, 2, 2, 2],
            vec![2, 0, 0, 2, 0]
        ]),
        5
    );
}
