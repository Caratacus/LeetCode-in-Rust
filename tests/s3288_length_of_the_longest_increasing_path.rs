// Tests for Problem 3288: Length of the Longest Increasing Path
// Java reference: src/test/java/g3201_3300/s3288_length_of_the_longest_increasing_path/SolutionTest.java

use leetcode_in_rust::s3288::length_of_the_longest_increasing_path::Solution;

#[test]
fn test_max_path_length() {
    assert_eq!(
        Solution::max_path_length(
            vec![vec![3, 1], vec![2, 2], vec![4, 1], vec![0, 0], vec![5, 3]],
            1
        ),
        3
    );
}

#[test]
fn test_max_path_length2() {
    assert_eq!(
        Solution::max_path_length(vec![vec![2, 1], vec![7, 0], vec![5, 6]], 2),
        2
    );
}

#[test]
fn test_max_path_length3() {
    assert_eq!(
        Solution::max_path_length(vec![vec![0, 3], vec![8, 5], vec![6, 8]], 0),
        2
    );
}

#[test]
fn test_max_path_length4() {
    assert_eq!(
        Solution::max_path_length(vec![vec![8, 8], vec![7, 0], vec![5, 6], vec![9, 1]], 0),
        2
    );
}

#[test]
fn test_max_path_length5() {
    assert_eq!(
        Solution::max_path_length(
            vec![
                vec![1, 1],
                vec![0, 1],
                vec![5, 4],
                vec![3, 3],
                vec![2, 0],
                vec![1, 4],
                vec![6, 8]
            ],
            6
        ),
        4
    );
}
