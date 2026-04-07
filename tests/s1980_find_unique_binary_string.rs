// Tests for Problem 1980: Find Unique Binary String
// Java reference: src/test/java/g1901_2000/s1980_find_unique_binary_string/SolutionTest.java

use leetcode_in_rust::s1980::find_unique_binary_string::Solution;

#[test]
fn test_find_different_binary_string() {
    assert_eq!(
        Solution::find_different_binary_string(vec![String::from("01"), String::from("10")]),
        String::from("00")
    );
}

#[test]
fn test_find_different_binary_string2() {
    assert_eq!(
        Solution::find_different_binary_string(vec![String::from("00"), String::from("01")]),
        String::from("10")
    );
}

#[test]
fn test_find_different_binary_string3() {
    assert_eq!(
        Solution::find_different_binary_string(vec![
            String::from("111"),
            String::from("011"),
            String::from("001")
        ]),
        String::from("000")
    );
}
