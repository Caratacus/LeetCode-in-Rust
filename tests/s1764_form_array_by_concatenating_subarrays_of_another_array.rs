// Tests for Problem 1764: Form Array by Concatenating Subarrays of Another Array
// Java reference: src/test/java/g1701_1800/s1764_form_array_by_concatenating_subarrays_of_another_array/SolutionTest.java

use leetcode_in_rust::s1764::form_array_by_concatenating_subarrays_of_another_array::Solution;

#[test]
fn test_can_choose() {
    assert_eq!(
        Solution::can_choose(
            vec![vec![1, -1, -1], vec![3, -2, 0]],
            vec![1, -1, 0, 1, -1, -1, 3, -2, 0]
        ),
        true
    );
}

#[test]
fn test_can_choose2() {
    assert_eq!(
        Solution::can_choose(
            vec![vec![10, -2], vec![1, 2, 3, 4]],
            vec![1, 2, 3, 4, 10, -2]
        ),
        false
    );
}

#[test]
fn test_can_choose3() {
    assert_eq!(
        Solution::can_choose(
            vec![vec![1, 2, 3], vec![3, 4]],
            vec![7, 7, 1, 2, 3, 4, 7, 7]
        ),
        false
    );
}
