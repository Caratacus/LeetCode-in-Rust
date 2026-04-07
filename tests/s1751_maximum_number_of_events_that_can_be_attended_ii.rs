// Tests for Problem 1751: Maximum Number of Events That Can Be Attended II
// Java reference: src/test/java/g1701_1800/s1751_maximum_number_of_events_that_can_be_attended_ii/SolutionTest.java

use leetcode_in_rust::s1751::maximum_number_of_events_that_can_be_attended_ii::Solution;

#[test]
fn test_max_value() {
    assert_eq!(
        Solution::max_value(
            vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 1]],
            2
        ),
        7
    );
}

#[test]
fn test_max_value2() {
    assert_eq!(
        Solution::max_value(
            vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 10]],
            2
        ),
        10
    );
}

#[test]
fn test_max_value3() {
    assert_eq!(
        Solution::max_value(
            vec![vec![1, 1, 1], vec![2, 2, 2], vec![3, 3, 3], vec![4, 4, 4]],
            3
        ),
        9
    );
}
