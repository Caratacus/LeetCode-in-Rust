// Tests for Problem 1439: Find the Kth Smallest Sum of a Matrix With Sorted Rows
// Java reference: src/test/java/g1401_1500/s1439_find_the_kth_smallest_sum_of_a_matrix_with_sorted_rows/SolutionTest.java

use leetcode_in_rust::s1439::find_the_kth_smallest_sum_of_a_matrix_with_sorted_rows::Solution;

#[test]
fn test_kth_smallest() {
    let mat = vec![
        vec![1, 3, 11],
        vec![2, 4, 6],
    ];
    assert_eq!(Solution::kth_smallest(mat, 5), 7);
}

#[test]
fn test_kth_smallest2() {
    let mat = vec![
        vec![1, 3, 11],
        vec![2, 4, 6],
    ];
    assert_eq!(Solution::kth_smallest(mat, 9), 17);
}

#[test]
fn test_kth_smallest3() {
    let mat = vec![
        vec![1, 10, 10],
        vec![1, 4, 5],
        vec![2, 3, 6],
    ];
    assert_eq!(Solution::kth_smallest(mat, 7), 9);
}
