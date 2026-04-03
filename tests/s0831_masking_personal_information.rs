// Tests for Problem 0831: Masking Personal Information
// Java reference: src/test/java/g0801_0900/s0831_masking_personal_information/SolutionTest.java

use leetcode_in_rust::s0831::masking_personal_information::Solution;

#[test]
fn test_mask_pii() {
    assert_eq!(
        Solution::mask_pii("LeetCode@LeetCode.com".to_string()),
        "l*****e@l*****e.com"
    );
}

#[test]
fn test_mask_pii2() {
    assert_eq!(
        Solution::mask_pii("AB@qq.com".to_string()),
        "a*****@q.com"
    );
}

#[test]
fn test_mask_pii3() {
    assert_eq!(
        Solution::mask_pii("1(234)567-890".to_string()),
        "***-***-7890"
    );
}
