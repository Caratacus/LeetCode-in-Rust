// Tests for Problem 2965: Find Missing and Repeated Values
// Java reference: src/test/java/g2901_3000/s2965_find_missing_and_repeated_values/SolutionTest.java

use leetcode_in_rust::s2965::find_missing_and_repeated_values::Solution;

#[test]
fn test_find_missing_and_repeated_values() {
    assert_eq!(
        Solution::find_missing_and_repeated_values(vec![vec![1, 3], vec![2, 2]]),
        vec![2, 4]
    );
}

#[test]
fn test_find_missing_and_repeated_values2() {
    assert_eq!(
        Solution::find_missing_and_repeated_values(vec![vec![9, 1, 7], vec![8, 9, 2], vec![3, 4, 6]]),
        vec![9, 5]
    );
}
