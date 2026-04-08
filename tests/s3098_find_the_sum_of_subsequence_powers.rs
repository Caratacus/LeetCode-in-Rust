// Tests for Problem 3098: Find the Sum of Subsequence Powers
// Java reference: src/test/java/g3001_3100/s3098_find_the_sum_of_subsequence_powers/SolutionTest.java

use leetcode_in_rust::s3098::find_the_sum_of_subsequence_powers::Solution;

#[test]
fn test_sum_of_powers() {
    assert_eq!(Solution::sum_of_powers(vec![1, 2, 3, 4], 3), 4);
}

#[test]
fn test_sum_of_powers2() {
    assert_eq!(Solution::sum_of_powers(vec![2, 2], 2), 0);
}

#[test]
fn test_sum_of_powers3() {
    assert_eq!(Solution::sum_of_powers(vec![4, 3, -1], 2), 10);
}
