// Tests for Problem 2442: Count Number of Distinct Integers After Reverse Operations
// Java reference: src/test/java/g2401_2500/s2442_count_number_of_distinct_integers_after_reverse_operations/SolutionTest.java

use leetcode_in_rust::s2442::count_number_of_distinct_integers_after_reverse_operations::Solution;

#[test]
fn test_count_distinct_integers() {
    assert_eq!(
        Solution::count_distinct_integers(vec![1, 13, 10, 12, 31]),
        6
    );
}

#[test]
fn test_count_distinct_integers2() {
    assert_eq!(Solution::count_distinct_integers(vec![2, 2, 2]), 1);
}
