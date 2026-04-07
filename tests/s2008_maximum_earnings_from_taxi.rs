// Tests for Problem 2008: Maximum Earnings From Taxi
// Java reference: src/test/java/g2001_2100/s2008_maximum_earnings_from_taxi/SolutionTest.java

use leetcode_in_rust::s2008::maximum_earnings_from_taxi::Solution;

#[test]
fn test_max_taxi_earnings() {
    assert_eq!(
        Solution::max_taxi_earnings(5, vec![vec![2, 5, 4], vec![1, 5, 1]]),
        7
    );
}

#[test]
fn test_max_taxi_earnings2() {
    assert_eq!(
        Solution::max_taxi_earnings(
            20,
            vec![
                vec![1, 6, 1],
                vec![3, 10, 2],
                vec![10, 12, 3],
                vec![11, 12, 2],
                vec![12, 15, 2],
                vec![13, 18, 1]
            ]
        ),
        6
    );
}
