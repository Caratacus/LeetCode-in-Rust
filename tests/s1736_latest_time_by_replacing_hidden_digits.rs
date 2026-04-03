// Tests for Problem 1736: Latest Time by Replacing Hidden Digits
// Java reference: src/test/java/g1701_1800/s1736_latest_time_by_replacing_hidden_digits/SolutionTest.java

use leetcode_in_rust::s1736::latest_time_by_replacing_hidden_digits::Solution;

#[test]
fn test_maximum_time() {
    assert_eq!(Solution::maximum_time("2?:?0".to_string()), "23:50");
}

#[test]
fn test_maximum_time2() {
    assert_eq!(Solution::maximum_time("0?:3?".to_string()), "09:39");
}

#[test]
fn test_maximum_time3() {
    assert_eq!(Solution::maximum_time("1?:22".to_string()), "19:22");
}

#[test]
fn test_maximum_time4() {
    assert_eq!(Solution::maximum_time("?4:00".to_string()), "14:00");
}

#[test]
fn test_maximum_time5() {
    assert_eq!(Solution::maximum_time("??:??".to_string()), "23:59");
}

#[test]
fn test_maximum_time6() {
    assert_eq!(Solution::maximum_time("?3:15".to_string()), "23:15");
}

#[test]
fn test_maximum_time7() {
    assert_eq!(Solution::maximum_time("2?:45".to_string()), "23:45");
}

#[test]
fn test_maximum_time8() {
    assert_eq!(Solution::maximum_time("1?:??".to_string()), "19:59");
}

#[test]
fn test_maximum_time9() {
    assert_eq!(Solution::maximum_time("10:?7".to_string()), "10:57");
}

#[test]
fn test_maximum_time10() {
    assert_eq!(Solution::maximum_time("22:4?".to_string()), "22:49");
}
