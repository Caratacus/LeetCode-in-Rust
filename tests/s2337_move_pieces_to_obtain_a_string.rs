// Tests for Problem 2337: Move Pieces to Obtain a String
// Java reference: src/test/java/g2301_2400/s2337_move_pieces_to_obtain_a_string/SolutionTest.java

use leetcode_in_rust::s2337::move_pieces_to_obtain_a_string::Solution;

#[test]
fn test_can_change() {
    assert_eq!(
        Solution::can_change("_L__R__R_".to_string(), "L______RR".to_string()),
        true
    );
}

#[test]
fn test_can_change2() {
    assert_eq!(
        Solution::can_change("R_L_".to_string(), "__LR".to_string()),
        false
    );
}

#[test]
fn test_can_change3() {
    assert_eq!(Solution::can_change("_R".to_string(), "R_".to_string()), false);
}

#[test]
fn test_can_change4() {
    assert_eq!(
        Solution::can_change("_L__R__R_L".to_string(), "L______RR_".to_string()),
        false
    );
}
