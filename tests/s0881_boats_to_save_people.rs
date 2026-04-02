// Tests for Problem 0881: Boats to Save People
// Java reference: src/test/java/g0801_0900/s0881_boats_to_save_people/SolutionTest.java

use leetcode_in_rust::s0881::boats_to_save_people::Solution;

#[test]
fn test_num_rescue_boats() {
    assert_eq!(Solution::num_rescue_boats(vec![1, 2], 3), 1);
}

#[test]
fn test_num_rescue_boats2() {
    assert_eq!(Solution::num_rescue_boats(vec![3, 2, 2, 1], 3), 3);
}

#[test]
fn test_num_rescue_boats3() {
    assert_eq!(Solution::num_rescue_boats(vec![3, 5, 3, 4], 5), 4);
}
