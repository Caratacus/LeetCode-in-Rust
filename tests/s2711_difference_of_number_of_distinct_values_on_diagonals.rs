// Tests for Problem 2711: Difference of Number of Distinct Values on Diagonals
// Java reference: src/test/java/g2701_2800/s2711_difference_of_number_of_distinct_values_on_diagonals/SolutionTest.java

use leetcode_in_rust::s2711::difference_of_number_of_distinct_values_on_diagonals::Solution;

#[test]
fn test_difference_of_distinct_values() {
    assert_eq!(
        Solution::difference_of_distinct_values(vec![vec![1, 2, 3], vec![3, 1, 5], vec![3, 2, 1]]),
        vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 1, 1]]
    );
}

#[test]
fn test_difference_of_distinct_values2() {
    assert_eq!(Solution::difference_of_distinct_values(vec![vec![1]]), vec![vec![0]]);
}
