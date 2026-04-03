// Tests for Problem 1521: Find a Value of a Mysterious Function Closest to Target
// Java reference: src/test/java/g1501_1600/s1521_find_a_value_of_a_mysterious_function_closest_to_target/SolutionTest.java

use leetcode_in_rust::s1521::find_a_value_of_a_mysterious_function_closest_to_target::Solution;

#[test]
fn test_closest_to_target() {
    assert_eq!(Solution::closest_to_target(vec![9, 12, 3, 7, 15], 5), 2);
}

#[test]
fn test_closest_to_target2() {
    assert_eq!(Solution::closest_to_target(vec![1000000, 1000000, 1000000], 1), 999999);
}

#[test]
fn test_closest_to_target3() {
    assert_eq!(Solution::closest_to_target(vec![1, 2, 4, 8, 16], 0), 0);
}
