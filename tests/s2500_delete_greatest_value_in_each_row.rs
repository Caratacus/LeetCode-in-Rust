// Tests for Problem 2500: Delete Greatest Value in Each Row
// Java reference: src/test/java/g2401_2500/s2500_delete_greatest_value_in_each_row/SolutionTest.java

use leetcode_in_rust::s2500::delete_greatest_value_in_each_row::Solution;

#[test]
fn test_delete_greatest_value() {
    assert_eq!(
        Solution::delete_greatest_value(vec![vec![1, 2, 4], vec![3, 3, 1]]),
        8
    );
}

#[test]
fn test_delete_greatest_value2() {
    assert_eq!(
        Solution::delete_greatest_value(vec![vec![10]]),
        10
    );
}
