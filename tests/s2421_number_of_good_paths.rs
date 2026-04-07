// Tests for Problem 2421: Number of Good Paths
// Java reference: src/test/java/g2401_2500/s2421_number_of_good_paths/SolutionTest.java

use leetcode_in_rust::s2421::number_of_good_paths::Solution;

#[test]
fn test_number_of_good_paths() {
    assert_eq!(
        Solution::number_of_good_paths(
            vec![1, 3, 2, 1, 3],
            vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]]
        ),
        6
    );
}

#[test]
fn test_number_of_good_paths2() {
    assert_eq!(
        Solution::number_of_good_paths(
            vec![1, 1, 2, 2, 3],
            vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![2, 4]]
        ),
        7
    );
}
