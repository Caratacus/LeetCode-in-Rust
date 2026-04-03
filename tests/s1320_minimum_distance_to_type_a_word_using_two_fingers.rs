// Tests for Problem 1320: Minimum Distance to Type a Word Using Two Fingers
// Java reference: src/test/java/g1301_1400/s1320_minimum_distance_to_type_a_word_using_two_fingers/SolutionTest.java

use leetcode_in_rust::s1320::minimum_distance_to_type_a_word_using_two_fingers::Solution;

#[test]
fn test_minimum_distance() {
    let result = Solution::minimum_distance("CAKE".to_string());
    assert_eq!(result, 3);
}

#[test]
fn test_minimum_distance2() {
    let result = Solution::minimum_distance("HAPPY".to_string());
    assert_eq!(result, 6);
}
