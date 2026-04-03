// Tests for Problem 1394: Find Lucky Integer in an Array
// Java reference: src/test/java/g1301_1400/s1394_find_lucky_integer_in_an_array/SolutionTest.java

use leetcode_in_rust::s1394::find_lucky_integer_in_an_array::Solution;

#[test]
fn test_find_lucky() {
    assert_eq!(Solution::find_lucky(vec![2, 2, 3, 4]), 2);
}

#[test]
fn test_find_lucky2() {
    assert_eq!(Solution::find_lucky(vec![1, 2, 2, 3, 3, 3]), 3);
}

#[test]
fn test_find_lucky3() {
    assert_eq!(Solution::find_lucky(vec![2, 2, 2, 3, 3]), -1);
}
