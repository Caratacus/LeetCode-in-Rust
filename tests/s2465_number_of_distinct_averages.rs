// Tests for Problem 2465: Number of Distinct Averages
// Java reference: src/test/java/g2401_2500/s2465_number_of_distinct_averages/SolutionTest.java

use leetcode_in_rust::s2465::number_of_distinct_averages::Solution;

#[test]
fn test_distinct_averages() {
    assert_eq!(Solution::distinct_averages(vec![4, 1, 4, 0, 3, 5]), 2);
}

#[test]
fn test_distinct_averages2() {
    assert_eq!(Solution::distinct_averages(vec![1, 100]), 1);
}
