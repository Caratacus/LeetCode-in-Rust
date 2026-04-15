// Tests for Problem 3377: Digit Operations to Make Two Integers Equal
// Java reference: src/test/java/g3301_3400/s3377_digit_operations_to_make_two_integers_equal/SolutionTest.java

use leetcode_in_rust::s3377::digit_operations_to_make_two_integers_equal::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(Solution::min_operations(10, 12), 85);
}

#[test]
fn test_min_operations2() {
    assert_eq!(Solution::min_operations(4, 8), -1);
}

#[test]
fn test_min_operations3() {
    assert_eq!(Solution::min_operations(6, 2), -1);
}

#[test]
fn test_min_operations4() {
    assert_eq!(Solution::min_operations(17, 72), -1);
}
