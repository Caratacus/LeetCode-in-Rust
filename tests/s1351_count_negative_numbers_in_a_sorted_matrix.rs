// Tests for Problem 1351: Count Negative Numbers in a Sorted Matrix
// Java reference: src/test/java/g1301_1400/s1351_count_negative_numbers_in_a_sorted_matrix/SolutionTest.java

use leetcode_in_rust::s1351::count_negative_numbers_in_a_sorted_matrix::Solution;

#[test]
fn test_count_negatives() {
    assert_eq!(
        Solution::count_negatives(vec![
            vec![4, 3, 2, -1],
            vec![3, 2, 1, -1],
            vec![1, 1, -1, -2],
            vec![-1, -1, -2, -3]
        ]),
        8
    );
}

#[test]
fn test_count_negatives2() {
    assert_eq!(Solution::count_negatives(vec![vec![3, 2], vec![1, 0]]), 0);
}
