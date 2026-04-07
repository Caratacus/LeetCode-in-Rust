// Tests for Problem 2712: Minimum Cost to Make All Characters Equal
// Java reference: src/test/java/g2701_2800/s2712_minimum_cost_to_make_all_characters_equal/SolutionTest.java

use leetcode_in_rust::s2712::minimum_cost_to_make_all_characters_equal::Solution;

#[test]
fn test_minimum_cost() {
    assert_eq!(Solution::minimum_cost("0011".to_string()), 2);
}

#[test]
fn test_minimum_cost2() {
    assert_eq!(Solution::minimum_cost("010101".to_string()), 9);
}
