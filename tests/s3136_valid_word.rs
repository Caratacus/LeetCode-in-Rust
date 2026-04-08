// Tests for Problem 3136: Valid Word
// Java reference: src/test/java/g3101_3200/s3136_valid_word/SolutionTest.java

use leetcode_in_rust::s3136::valid_word::Solution;
#[test]
fn test_is_valid() {
    assert_eq!(Solution::is_valid(String::from("234Adas")), true);
}
#[test]
fn test_is_valid2() {
    assert_eq!(Solution::is_valid(String::from("b3")), false);
}
#[test]
fn test_is_valid3() {
    assert_eq!(Solution::is_valid(String::from("a3$e")), false);
}
#[test]
fn test_is_valid4() {
    assert_eq!(Solution::is_valid(String::from("a")), false);
    assert_eq!(Solution::is_valid(String::from("ab")), false);
    assert_eq!(Solution::is_valid(String::from("1")), false);
    assert_eq!(Solution::is_valid(String::from("1a")), false);
    assert_eq!(Solution::is_valid(String::from("")), false);
}
#[test]
fn test_is_valid5() {
    assert_eq!(Solution::is_valid(String::from("aei")), false);
    assert_eq!(Solution::is_valid(String::from("AEI")), false);
    assert_eq!(Solution::is_valid(String::from("Aei"), false);
    assert_eq!(Solution::is_valid(String::from("uuu"), false);
}
#[test]
fn test_is_valid6() {
    assert_eq!(Solution::is_valid(String::from("bcdfg")), false);
    assert_eq!(Solution::is_valid(String::from("BCD"), false);
    assert_eq!(Solution::is_valid(String::from("xyz"), false);
    assert_eq!(Solution::is_valid(String::from("QWRTY"), false);
}
#[test]
fn test_is_valid7() {
    assert_eq!(Solution::is_valid(String::from("abc")), true);
    assert_eq!(Solution::is_valid(String::from("bac"), true);
    assert_eq!(Solution::is_valid(String::from("AeIbcD"), true);
    assert_eq!(Solution::is_valid(String::from("tree"), true);
    assert_eq!(Solution::is_valid(String::from("skyE"), true);
}
#[test]
fn test_is_valid8() {
    assert_eq!(Solution::is_valid(String::from("a1b2c"), true);
    assert_eq!(Solution::is_valid(String::from("1a2b"), true);
    assert_eq!(Solution::is_valid(String::from("b2c4e"), true);
    assert_eq!(Solution::is_valid(String::from("123"), false);
}
#[test]
fn test_is_valid10() {
    assert_eq!(Solution::is_valid(String::from("a#b"), false);
    assert_eq!(Solution::is_valid(String::from("@ab"), false);
    assert_eq!(Solution::is_valid(String::from("ab!"), false);
    assert_eq!(Solution::is_valid(String::from("c_d"), false);
    assert_eq!(Solution::is_valid(String::from("a.b"), false);
}
#[test]
fn test_is_valid11() {
    assert_eq!(Solution::is_valid(String::from("AbC"), true);
    assert_eq!(Solution::is_valid(String::from("BacE1"), true);
    assert_eq!(Solution::is_valid(String::from("zEi"), true);
}
