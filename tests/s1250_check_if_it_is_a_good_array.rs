// Tests for Problem 1250: Check If It Is a Good Array
// Java reference: src/test/java/g1201_1300/s1250_check_if_it_is_a_good_array/SolutionTest.java

use leetcode_in_rust::s1250::check_if_it_is_a_good_array::Solution;

#[test]
fn test_is_good_array() {
    assert_eq!(Solution::is_good_array(vec![12, 5, 7, 23]), true);
}

#[test]
fn test_is_good_array2() {
    assert_eq!(Solution::is_good_array(vec![29, 6, 10]), true);
}

#[test]
fn test_is_good_array3() {
    assert_eq!(Solution::is_good_array(vec![3, 6]), false);
}
