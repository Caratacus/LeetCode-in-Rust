// Tests for Problem 0794: Valid Tic-Tac-Toe State
// Java reference: src/test/java/g0701_0800/s0794_valid_tic_tac_toe_state/SolutionTest.java

use leetcode_in_rust::s0794::valid_tic_tac_toe_state::Solution;

#[test]
fn test_valid_tic_tac_toe() {
    assert_eq!(
        Solution::valid_tic_tac_toe(vec!["O  ".to_string(), "   ".to_string(), "   ".to_string()]),
        false
    );
}

#[test]
fn test_valid_tic_tac_toe2() {
    assert_eq!(
        Solution::valid_tic_tac_toe(vec!["XOX".to_string(), " X ".to_string(), "   ".to_string()]),
        false
    );
}

#[test]
fn test_valid_tic_tac_toe3() {
    assert_eq!(
        Solution::valid_tic_tac_toe(vec!["XOX".to_string(), "O O".to_string(), "XOX".to_string()]),
        true
    );
}
