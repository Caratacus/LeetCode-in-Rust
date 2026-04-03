// Tests for Problem 1467: Probability of a Two Boxes Having The Same Number of Distinct Balls
// Java reference: src/test/java/g1401_1500/s1467_probability_of_a_two_boxes_having_the_same_number_of_distinct_balls/SolutionTest.java

use leetcode_in_rust::s1467::probability_of_a_two_boxes_having_the_same_number_of_distinct_balls::Solution;

#[test]
fn test_get_probability() {
    let result = Solution::get_probability(vec![1, 1]);
    assert!((result - 1.0).abs() < 1e-5);
}

#[test]
fn test_get_probability2() {
    let result = Solution::get_probability(vec![2, 1, 1]);
    assert!((result - 0.6666666666666666).abs() < 1e-5);
}

#[test]
fn test_get_probability3() {
    let result = Solution::get_probability(vec![1, 2, 1, 2]);
    assert!((result - 0.6).abs() < 1e-5);
}
