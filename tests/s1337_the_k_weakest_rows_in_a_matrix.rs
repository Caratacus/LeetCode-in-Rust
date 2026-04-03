// Tests for Problem 1337: The K Weakest Rows in a Matrix
// Java reference: src/test/java/g1301_1400/s1337_the_k_weakest_rows_in_a_matrix/SolutionTest.java

use leetcode_in_rust::s1337::the_k_weakest_rows_in_a_matrix::Solution;

#[test]
fn test_k_weakest_rows() {
    assert_eq!(
        Solution::k_weakest_rows(
            vec![
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 1, 1, 1]
            ],
            3
        ),
        vec![2, 0, 3]
    );
}

#[test]
fn test_k_weakest_rows2() {
    assert_eq!(
        Solution::k_weakest_rows(
            vec![vec![1, 0, 0, 0], vec![1, 1, 1, 1], vec![1, 0, 0, 0], vec![1, 0, 0, 0]],
            2
        ),
        vec![0, 2]
    );
}
