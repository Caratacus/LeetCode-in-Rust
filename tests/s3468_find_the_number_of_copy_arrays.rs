// Tests for Problem 3468: Find the Number of Copy Arrays
// Java reference: src/test/java/g3401_3500/s3468_find_the_number_of_copy_arrays/SolutionTest.java

use leetcode_in_rust::s3468::find_the_number_of_copy_arrays::Solution;

#[test]
fn test_count_arrays() {
    assert_eq!(
        Solution::count_arrays(vec![1, 2, 3, 4], vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]),
        2
    );
}

#[test]
fn test_count_arrays2() {
    assert_eq!(
        Solution::count_arrays(vec![1, 2, 3, 4], vec![vec![1, 10], vec![2, 9], vec![3, 8], vec![4, 7]]),
        4
    );
}

#[test]
fn test_count_arrays3() {
    assert_eq!(
        Solution::count_arrays(vec![1, 2, 1, 2], vec![vec![1, 1], vec![2, 3], vec![3, 3], vec![2, 3]]),
        0
    );
}
