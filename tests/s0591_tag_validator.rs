// Tests for Problem 0591: Tag Validator
// Java reference: src/test/java/g0501_0600/s0591_tag_validator/SolutionTest.java

use leetcode_in_rust::s0591::tag_validator::Solution;

#[test]
fn test_is_valid() {
    assert_eq!(
        Solution::is_valid("<DIV>This is the first line <![CDATA[<div>]]></DIV>".to_string()),
        true
    );
}

#[test]
fn test_is_valid2() {
    assert_eq!(
        Solution::is_valid(
            "<ABCDEFGHIJKLMN>This is the first line <![CDATA[<div>]]></ABCDEFGHIJKLMN>".to_string()
        ),
        false
    );
}
