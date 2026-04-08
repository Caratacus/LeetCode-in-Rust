// Tests for Problem 3261: Count Substrings That Satisfy K-Constraint II
// Java reference: src/test/java/g3201_3300/s3261_count_substrings_that_satisfy_k_constraint_ii/SolutionTest.java

use leetcode_in_rust::s3261::count_substrings_that_satisfy_k_constraint_ii::Solution;

#[test]
fn test_count_k_constraint_substrings() {
    assert_eq!(
        Solution::count_k_constraint_substrings("0001111".to_string(), 2, vec![vec![0, 6]]),
        vec![26]
    );
}

#[test]
fn test_count_k_constraint_substrings2() {
    assert_eq!(
        Solution::count_k_constraint_substrings(
            "010101".to_string(),
            1,
            vec![vec![0, 5], vec![1, 4], vec![2, 3]]
        ),
        vec![15, 9, 3]
    );
}
