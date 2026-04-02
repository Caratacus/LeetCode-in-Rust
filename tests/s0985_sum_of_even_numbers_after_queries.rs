// Tests for Problem 0985: Sum of Even Numbers After Queries
// Java reference: src/test/java/g0901_1000/s0985_sum_of_even_numbers_after_queries/SolutionTest.java

use leetcode_in_rust::s0985::sum_of_even_numbers_after_queries::Solution;

#[test]
fn test_sum_even_after_queries() {
    assert_eq!(
        Solution::sum_even_after_queries(
            vec![1, 2, 3, 4],
            vec![vec![1, 0], vec![-3, 1], vec![-4, 0], vec![2, 3]]
        ),
        vec![8, 6, 2, 4]
    );
}

#[test]
fn test_sum_even_after_queries2() {
    assert_eq!(
        Solution::sum_even_after_queries(vec![1], vec![vec![4, 0]]),
        vec![0]
    );
}
