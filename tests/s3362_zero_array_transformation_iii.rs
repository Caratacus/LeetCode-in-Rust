// Tests for Problem 3362: Zero Array Transformation III
// Java reference: src/test/java/g3301_3400/s3362_zero_array_transformation_iii/SolutionTest.java

use leetcode_in_rust::s3362::zero_array_transformation_iii::Solution;

#[test]
fn test_max_removal() {
    assert_eq!(
        Solution::max_removal(vec![2, 0, 2], vec![vec![0, 2], vec![0, 2], vec![1, 1]]),
        1
    );
}

#[test]
fn test_max_removal2() {
    assert_eq!(
        Solution::max_removal(vec![1, 1, 1, 1], vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![1, 2]]),
        2
    );
}

#[test]
fn test_max_removal3() {
    assert_eq!(
        Solution::max_removal(vec![1, 2, 3, 4], vec![vec![0, 3]]),
        -1
    );
}
