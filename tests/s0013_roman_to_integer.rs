// Tests for Problem 0013: Roman to Integer
// Java reference: src/test/java/g0001_0100/s0013_roman_to_integer/SolutionTest.java

use leetcode_in_rust::s0013::roman_to_integer::Solution;

#[test]
fn test_roman_to_int() {
    assert_eq!(Solution::roman_to_int("III".to_string()), 3);
}

#[test]
fn test_roman_to_int2() {
    assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
}

#[test]
fn test_roman_to_int3() {
    assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
}

#[test]
fn test_roman_to_int4() {
    assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
}

#[test]
fn test_roman_to_int5() {
    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
}
