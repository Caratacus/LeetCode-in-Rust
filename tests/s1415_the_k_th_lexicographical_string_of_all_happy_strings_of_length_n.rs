// Tests for Problem 1415: The k-th Lexicographical String of All Happy Strings of Length n
// Java reference: src/test/java/g1401_1500/s1415_the_k_th_lexicographical_string_of_all_happy_strings_of_length_n/SolutionTest.java

use leetcode_in_rust::s1415::the_k_th_lexicographical_string_of_all_happy_strings_of_length_n::Solution;

#[test]
fn test_get_happy_string() {
    assert_eq!(Solution::get_happy_string(1, 3), "c");
}

#[test]
fn test_get_happy_string2() {
    assert_eq!(Solution::get_happy_string(1, 4), "");
}

#[test]
fn test_get_happy_string3() {
    assert_eq!(Solution::get_happy_string(3, 9), "cab");
}
