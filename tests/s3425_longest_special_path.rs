// Tests for Problem 3425: Longest Special Path
// Java reference: src/test/java/g3401_3500/s3425_longest_special_path/SolutionTest.java

use leetcode_in_rust::s3425::longest_special_path::Solution;

#[test]
fn test_longest_special_path() {
    assert_eq!(
        Solution::longest_special_path(
            vec![vec![0, 1, 2], vec![1, 2, 3], vec![1, 3, 5], vec![1, 4, 4], vec![2, 5, 6]],
            vec![2, 1, 2, 1, 3, 1]
        ),
        vec![6, 2]
    );
}

#[test]
fn test_longest_special_path2() {
    assert_eq!(
        Solution::longest_special_path(vec![vec![1, 0, 8]], vec![2, 2]),
        vec![0, 1]
    );
}
