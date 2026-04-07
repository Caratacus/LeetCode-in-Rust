// Tests for Problem 2836: Maximize Value of Function in a Ball Passing Game
// Java reference: src/test/java/g2801_2900/s2836_maximize_value_of_function_in_a_ball_passing_game/SolutionTest.java

use leetcode_in_rust::s2836::maximize_value_of_function_in_a_ball_passing_game::Solution;

#[test]
fn test_get_max_function_value() {
    assert_eq!(Solution::get_max_function_value(vec![2, 0, 1], 4), 6);
}

#[test]
fn test_get_max_function_value2() {
    assert_eq!(Solution::get_max_function_value(vec![1, 1, 1, 2, 3], 3), 10);
}
