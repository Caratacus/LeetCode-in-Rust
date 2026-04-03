// Tests for Problem 1728: Cat and Mouse II
// Java reference: src/test/java/g1701_1800/s1728_cat_and_mouse_ii/SolutionTest.java

use leetcode_in_rust::s1728::cat_and_mouse_ii::Solution;

#[test]
fn test_can_mouse_win() {
    assert_eq!(
        Solution::can_mouse_win(vec!["####F".to_string(), "#C...".to_string(), "M....".to_string()], 1, 2),
        true
    );
}

#[test]
fn test_can_mouse_win2() {
    assert_eq!(
        Solution::can_mouse_win(vec!["M.C...F".to_string()], 1, 4),
        true
    );
}

#[test]
fn test_can_mouse_win3() {
    assert_eq!(
        Solution::can_mouse_win(vec!["M.C...F".to_string()], 1, 3),
        false
    );
}
