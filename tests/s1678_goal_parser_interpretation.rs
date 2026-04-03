// Tests for Problem 1678: Goal Parser Interpretation
// Java reference: src/test/java/g1601_1700/s1678_goal_parser_interpretation/SolutionTest.java

use leetcode_in_rust::s1678::goal_parser_interpretation::Solution;

#[test]
fn test_interpret() {
    assert_eq!(Solution::interpret("G()(al)".to_string()), "Goal".to_string());
}

#[test]
fn test_interpret2() {
    assert_eq!(Solution::interpret("G()()()()(al)".to_string()), "Gooooal".to_string());
}

#[test]
fn test_interpret3() {
    assert_eq!(Solution::interpret("(al)G(al)()()G".to_string()), "alGalooG".to_string());
}
