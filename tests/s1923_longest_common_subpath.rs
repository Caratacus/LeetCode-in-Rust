// Tests for Problem 1923: Longest Common Subpath
// Java reference: src/test/java/g1901_2000/s1923_longest_common_subpath/SolutionTest.java

use leetcode_in_rust::s1923::longest_common_subpath::Solution;

#[test]
fn test_longest_common_subpath() {
    assert_eq!(
        Solution::longest_common_subpath(
            5,
            vec![vec![0, 1, 2, 3, 4], vec![2, 3, 4], vec![4, 0, 1, 2, 3]]
        ),
        2
    );
}

#[test]
fn test_longest_common_subpath2() {
    assert_eq!(
        Solution::longest_common_subpath(5, vec![vec![0], vec![1], vec![2]]),
        0
    );
}

#[test]
fn test_longest_common_subpath3() {
    assert_eq!(
        Solution::longest_common_subpath(5, vec![vec![0, 1, 2, 3, 4], vec![4, 3, 2, 1, 0]]),
        1
    );
}
