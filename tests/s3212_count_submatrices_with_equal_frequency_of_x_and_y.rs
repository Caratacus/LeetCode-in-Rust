// Tests for Problem 3212: Count Submatrices With Equal Frequency of X and Y
// Java reference: src/test/java/g3201_3300/s3212_count_submatrices_with_equal_frequency_of_x_and_y/SolutionTest.java

use leetcode_in_rust::s3212::count_submatrices_with_equal_frequency_of_x_and_y::Solution;

#[test]
fn test_number_of_submatrices() {
    assert_eq!(
        Solution::number_of_submatrices(vec![
            vec!['X', 'Y', '.'],
            vec!['Y', '.', '.']
        ]),
        3
    );
}

#[test]
fn test_number_of_submatrices2() {
    assert_eq!(
        Solution::number_of_submatrices(vec![
            vec!['X', 'X'],
            vec!['X', 'Y']
        ]),
        0
    );
}

#[test]
fn test_number_of_submatrices3() {
    assert_eq!(
        Solution::number_of_submatrices(vec![
            vec!['.', '.'],
            vec!['.', '.']
        ]),
        0
    );
}
