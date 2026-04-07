// Tests for Problem 1955: Count Number of Special Subsequences
// Java reference: src/test/java/g1901_2000/s1955_count_number_of_special_subsequences/SolutionTest.java

use leetcode_in_rust::s1955::count_number_of_special_subsequences::Solution;

#[test]
fn test_count_special_subsequences() {
    assert_eq!(Solution::count_special_subsequences(vec![0, 1, 2, 2]), 3);
}

#[test]
fn test_count_special_subsequences2() {
    assert_eq!(Solution::count_special_subsequences(vec![2, 2, 0, 0]), 0);
}

#[test]
fn test_count_special_subsequences3() {
    assert_eq!(Solution::count_special_subsequences(vec![0, 1, 2, 0, 1, 2]), 7);
}
