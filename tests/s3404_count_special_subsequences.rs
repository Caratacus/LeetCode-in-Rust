// Tests for Problem 3404: Count Special Subsequences
// Java reference: src/test/java/g3401_3500/s3404_count_special_subsequences/SolutionTest.java

use leetcode_in_rust::s3404::count_special_subsequences::Solution;

#[test]
fn test_number_of_subsequences() {
    assert_eq!(
        Solution::number_of_subsequences(vec![1, 2, 3, 4, 3, 6, 1]),
        1
    );
}

#[test]
fn test_number_of_subsequences2() {
    assert_eq!(
        Solution::number_of_subsequences(vec![3, 4, 3, 4, 3, 4, 3, 4]),
        3
    );
}
