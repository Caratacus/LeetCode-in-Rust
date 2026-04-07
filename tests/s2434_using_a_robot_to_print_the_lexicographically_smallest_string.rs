// Tests for Problem 2434: Using a Robot to Print the Lexicographically Smallest String
// Java reference: src/test/java/g2401_2500/s2434_using_a_robot_to_print_the_lexicographically_smallest_string/SolutionTest.java

use leetcode_in_rust::s2434::using_a_robot_to_print_the_lexicographically_smallest_string::Solution;

#[test]
fn test_robot_with_string() {
    assert_eq!(Solution::robot_with_string("zza".to_string()), "azz");
}

#[test]
fn test_robot_with_string2() {
    assert_eq!(Solution::robot_with_string("bac".to_string()), "abc");
}

#[test]
fn test_robot_with_string3() {
    assert_eq!(Solution::robot_with_string("bdda".to_string()), "addb");
}
