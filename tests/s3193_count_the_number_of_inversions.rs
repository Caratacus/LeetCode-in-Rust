// Tests for Problem 3193: Count the Number of Inversions
// Java reference: src/test/java/g3101_3200/s3193_count_the_number_of_inversions/SolutionTest.java

use leetcode_in_rust::s3193::count_the_number_of_inversions::Solution;

#[test]
fn test_number_of_permutations() {
    assert_eq!(Solution::number_of_permutations(3, vec![vec![2, 2], vec![0, 0]]), 2);
}

#[test]
fn test_number_of_permutations2() {
    assert_eq!(Solution::number_of_permutations(3, vec![vec![2, 2], vec![1, 1], vec![0, 0]]), 1);
}

#[test]
fn test_number_of_permutations3() {
    assert_eq!(Solution::number_of_permutations(2, vec![vec![0, 0], vec![1, 0]]), 1);
}
