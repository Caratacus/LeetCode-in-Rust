// Tests for Problem 1277: Count Square Submatrices with All Ones
// Java reference: src/test/java/g1201_1300/s1277_count_square_submatrices_with_all_ones/SolutionTest.java

use leetcode_in_rust::s1277::count_square_submatrices_with_all_ones::Solution;

#[test]
fn test_count_squares() {
    assert_eq!(
        Solution::count_squares(vec![
            vec![0, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![0, 1, 1, 1]
        ]),
        15
    );
}

#[test]
fn test_count_squares2() {
    assert_eq!(
        Solution::count_squares(vec![
            vec![1, 0, 1],
            vec![1, 1, 0],
            vec![1, 1, 0]
        ]),
        7
    );
}
