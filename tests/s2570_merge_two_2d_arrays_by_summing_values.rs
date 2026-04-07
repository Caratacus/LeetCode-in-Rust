// Tests for Problem 2570: Merge Two 2D Arrays by Summing Values
// Java reference: src/test/java/g2501_2600/s2570_merge_two_2d_arrays_by_summing_values/SolutionTest.java

use leetcode_in_rust::s2570::merge_two_2d_arrays_by_summing_values::Solution;

#[test]
fn test_merge_arrays() {
    assert_eq!(
        Solution::merge_arrays(
            vec![vec![1, 2], vec![2, 3], vec![4, 5]],
            vec![vec![1, 4], vec![3, 2], vec![4, 1]]
        ),
        vec![vec![1, 6], vec![2, 3], vec![3, 2], vec![4, 6]]
    );
}

#[test]
fn test_merge_arrays2() {
    assert_eq!(
        Solution::merge_arrays(
            vec![vec![2, 4], vec![3, 6], vec![5, 5]],
            vec![vec![1, 3], vec![4, 3]]
        ),
        vec![vec![1, 3], vec![2, 4], vec![3, 6], vec![4, 3], vec![5, 5]]
    );
}
