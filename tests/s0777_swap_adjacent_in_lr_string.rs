// Tests for Problem 0777: Swap Adjacent in LR String
// Java reference: src/test/java/g0701_0800/s0777_swap_adjacent_in_lr_string/SolutionTest.java

use leetcode_in_rust::s0777::swap_adjacent_in_lr_string::Solution;

#[test]
fn test_can_transform() {
    assert_eq!(
        Solution::can_transform("RXXLRXRXL".to_string(), "XRLXXRRLX".to_string()),
        true
    );
}

#[test]
fn test_can_transform2() {
    assert_eq!(
        Solution::can_transform("X".to_string(), "L".to_string()),
        false
    );
}
