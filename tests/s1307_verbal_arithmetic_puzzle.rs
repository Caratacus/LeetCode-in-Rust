// Tests for Problem 1307: Verbal Arithmetic Puzzle
// Java reference: src/test/java/g1301_1400/s1307_verbal_arithmetic_puzzle/SolutionTest.java

use leetcode_in_rust::s1307::verbal_arithmetic_puzzle::Solution;

#[test]
fn test_is_solvable() {
    let result = Solution::is_solvable(vec!["SEND".to_string(), "MORE".to_string()], "MONEY".to_string());
    assert_eq!(result, true);
}

#[test]
fn test_is_solvable2() {
    let result = Solution::is_solvable(vec!["SIX".to_string(), "SEVEN".to_string(), "SEVEN".to_string()], "TWENTY".to_string());
    assert_eq!(result, true);
}

#[test]
fn test_is_solvable3() {
    let result = Solution::is_solvable(vec!["LEET".to_string(), "CODE".to_string()], "POINT".to_string());
    assert_eq!(result, false);
}
