// Tests for Problem 2529: Maximum Count Of Positive Integer And Negative Integer
// Java reference: src/test/java/g2501_2600/s2529_maximum_count_of_positive_integer_and_negative_integer/SolutionTest.java

use leetcode_in_rust::s2529::maximum_count_of_positive_integer_and_negative_integer::Solution;

#[test]
fn test_maximum_count() {
    assert_eq!(Solution::maximum_count(vec![-2, -1, -1, 1, 2, 3]), 3);
}

#[test]
fn test_maximum_count2() {
    assert_eq!(Solution::maximum_count(vec![-3, -2, -1, 0, 0, 5]), 3);
}
#[test]
fn test_maximum_count3() {
    assert_eq!(Solution::maximum_count(vec![5, 20, 66, 1314]), 4);
}
