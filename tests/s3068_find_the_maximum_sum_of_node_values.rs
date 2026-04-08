// Tests for Problem 3068: Find the Maximum Sum of Node Values
// Java reference: src/test/java/g3001_3100/s3068_find_the_maximum_sum_of_node_values/SolutionTest.java

use leetcode_in_rust::s3068::find_the_maximum_sum_of_node_values::Solution;

#[test]
fn test_maximum_value_sum() {
    assert_eq!(
        Solution::maximum_value_sum(vec![1, 2, 1], 3, vec![vec![0, 1], vec![0, 2]]),
        6
    );
}

#[test]
fn test_maximum_value_sum2() {
    assert_eq!(
        Solution::maximum_value_sum(vec![2, 3], 7, vec![vec![0, 1]]),
        9
    );
}

#[test]
fn test_maximum_value_sum3() {
    assert_eq!(
        Solution::maximum_value_sum(
            vec![7, 7, 7, 7, 7, 7],
            3,
            vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5]]
        ),
        42
    );
}
