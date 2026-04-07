// Tests for Problem 1957: Delete Characters to Make Fancy String
// Java reference: src/test/java/g1901_2000/s1957_delete_characters_to_make_fancy_string/SolutionTest.java

use leetcode_in_rust::s1957::delete_characters_to_make_fancy_string::Solution;

#[test]
fn test_make_fancy_string() {
    assert_eq!(Solution::make_fancy_string("leeetcode".to_string()), "leetcode");
}

#[test]
fn test_make_fancy_string2() {
    assert_eq!(Solution::make_fancy_string("aaabaaaa".to_string()), "aabaa");
}

#[test]
fn test_make_fancy_string3() {
    assert_eq!(Solution::make_fancy_string("aab".to_string()), "aab");
}
