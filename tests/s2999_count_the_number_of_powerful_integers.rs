// Tests for Problem 2999: Count the Number of Powerful Integers
// Java reference: src/test/java/g2901_3000/s2999_count_the_number_of_powerful_integers/SolutionTest.java

use leetcode_in_rust::s2999::count_the_number_of_powerful_integers::Solution;

#[test]
fn test_number_of_powerful_int() {
    assert_eq!(Solution::number_of_powerful_int(1, 6000, 4, "124".to_string()), 5);
}

#[test]
fn test_number_of_powerful_int2() {
    assert_eq!(Solution::number_of_powerful_int(15, 215, 5, "10".to_string()), 2);
}

#[test]
fn test_number_of_powerful_int3() {
    assert_eq!(Solution::number_of_powerful_int(1, 2000, 8, "1".to_string()), 162);
}
