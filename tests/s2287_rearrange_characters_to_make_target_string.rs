// Tests for Problem 2287: Rearrange Characters to Make Target String
// Java reference: src/test/java/g2201_2300/s2287_rearrange_characters_to_make_target_string/SolutionTest.java

use leetcode_in_rust::s2287::rearrange_characters_to_make_target_string::Solution;

#[test]
fn test_rearrange_characters() {
    assert_eq!(
        Solution::rearrange_characters(String::from("abcba"), String::from("abc")),
        1
    );
}

#[test]
fn test_rearrange_characters2() {
    assert_eq!(
        Solution::rearrange_characters(String::from("abbaccaddaeea"), String::from("aaaaa")),
        1
    );
}
