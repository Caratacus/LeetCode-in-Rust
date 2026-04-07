// Tests for Problem 2333: Minimum Sum of Squared Difference
// Java reference: src/test/java/g2301_2400/s2333_minimum_sum_of_squared_difference/SolutionTest.java

use leetcode_in_rust::s2333::minimum_sum_of_squared_difference::Solution;

#[test]
fn test_min_sum_square_diff() {
    assert_eq!(
        Solution::min_sum_square_diff(vec![1, 2, 3, 4], vec![2, 10, 20, 19], 0, 0),
        579
    );
}

#[test]
fn test_min_sum_square_diff2() {
    assert_eq!(
        Solution::min_sum_square_diff(vec![1, 4, 10, 12], vec![5, 8, 6, 9], 1, 1),
        43
    );
}

#[test]
fn test_min_sum_square_diff3() {
    assert_eq!(
        Solution::min_sum_square_diff(
            vec![7, 11, 4, 19, 11, 5, 6, 1, 8],
            vec![4, 7, 6, 16, 12, 9, 10, 2, 10],
            3,
            6
        ),
        27
    );
}
