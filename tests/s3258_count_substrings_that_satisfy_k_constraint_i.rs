// Tests for Problem 3258: Count Substrings That Satisfy K-Constraint I
// Java reference: src/test/java/g3201_3300/s3258_count_substrings_that_satisfy_k_constraint_i/SolutionTest.java

use leetcode_in_rust::s3258::count_substrings_that_satisfy_k_constraint_i::Solution;

#[test]
fn test_count_k_constraint_substrings() {
    assert_eq!(Solution::count_k_constraint_substrings("10101".to_string(), 1), 12);
}

#[test]
fn test_count_k_constraint_substrings2() {
    assert_eq!(Solution::count_k_constraint_substrings("1010101".to_string(), 2), 25);
}
