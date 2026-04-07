// Tests for Problem 1979: Find Greatest Common Divisor of Array
// Java reference: src/test/java/g1901_2000/s1979_find_greatest_common_divisor_of_array/SolutionTest.java

use leetcode_in_rust::s1979::find_greatest_common_divisor_of_array::Solution;

#[test]
fn test_find_gcd() {
    assert_eq!(Solution::find_gcd(vec![2, 5, 6, 9, 10]), 2);
}

#[test]
fn test_find_gcd2() {
    assert_eq!(Solution::find_gcd(vec![7, 5, 6, 8, 3]), 1);
}

#[test]
fn test_find_gcd3() {
    assert_eq!(Solution::find_gcd(vec![3, 3]), 3);
}
