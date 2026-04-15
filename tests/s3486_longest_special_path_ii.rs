// Tests for Problem 3486: Longest Special Path II
// Java reference: src/test/java/g3401_3500/s3486_longest_special_path_ii/SolutionTest.java

use leetcode_in_rust::s3486::longest_special_path_ii::Solution;

#[test]
fn test_longest_special_path() {
    assert_eq!(
        Solution::longest_special_path(
            vec![vec![0, 1, 1], vec![1, 2, 3], vec![1, 3, 1], vec![2, 4, 6], vec![4, 7, 2], vec![3, 5, 2], vec![3, 6, 5], vec![6, 8, 3]],
            vec![1, 1, 0, 3, 1, 2, 1, 1, 0]
        ),
        vec![9, 3]
    );
}

#[test]
fn test_longest_special_path2() {
    assert_eq!(
        Solution::longest_special_path(
            vec![vec![1, 0, 3], vec![0, 2, 4], vec![0, 3, 5]],
            vec![1, 1, 0, 2]
        ),
        vec![5, 2]
    );
}

#[test]
fn test_longest_special_path3() {
    assert_eq!(
        Solution::longest_special_path(
            vec![vec![0, 2, 4], vec![1, 2, 10], vec![3, 1, 5]],
            vec![4, 5, 4, 5]
        ),
        vec![15, 3]
    );
}
