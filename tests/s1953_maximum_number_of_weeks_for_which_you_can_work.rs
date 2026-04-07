// Tests for Problem 1953: Maximum Number of Weeks for Which You Can Work
// Java reference: src/test/java/g1901_2000/s1953_maximum_number_of_weeks_for_which_you_can_work/SolutionTest.java

use leetcode_in_rust::s1953::maximum_number_of_weeks_for_which_you_can_work::Solution;

#[test]
fn test_number_of_weeks() {
    assert_eq!(Solution::number_of_weeks(vec![1, 2, 3]), 6);
}

#[test]
fn test_number_of_weeks2() {
    assert_eq!(Solution::number_of_weeks(vec![5, 2, 1]), 7);
}
