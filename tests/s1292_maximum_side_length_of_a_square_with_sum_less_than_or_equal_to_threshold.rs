// Tests for Problem 1292: Maximum Side Length of a Square with Sum Less than or Equal to Threshold
// Java reference: src/test/java/g1201_1300/s1292_maximum_side_length_of_a_square_with_sum_less_than_or_equal_to_threshold/SolutionTest.java

use leetcode_in_rust::s1292::maximum_side_length_of_a_square_with_sum_less_than_or_equal_to_threshold::Solution;

#[test]
fn test_max_side_length() {
    assert_eq!(
        Solution::max_side_length(
            vec![
                vec![1, 1, 3, 2, 4, 3, 2],
                vec![1, 1, 3, 2, 4, 3, 2],
                vec![1, 1, 3, 2, 4, 3, 2]
            ],
            4
        ),
        2
    );
}

#[test]
fn test_max_side_length2() {
    assert_eq!(
        Solution::max_side_length(
            vec![
                vec![2, 2, 2, 2, 2],
                vec![2, 2, 2, 2, 2],
                vec![2, 2, 2, 2, 2],
                vec![2, 2, 2, 2, 2],
                vec![2, 2, 2, 2, 2]
            ],
            1
        ),
        0
    );
}
