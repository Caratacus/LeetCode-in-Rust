// Tests for Problem 3309: Maximum Possible Number by Binary Concatenation
// Java reference: src/test/java/g3301_3400/s3309_maximum_possible_number_by_binary_concatenation/SolutionTest.java

use leetcode_in_rust::s3309::maximum_possible_number_by_binary_concatenation::Solution;

#[test]
fn test_max_good_number() {
    assert_eq!(Solution::max_good_number(vec![1, 2, 3]), 30);
}

#[test]
fn test_max_good_number2() {
    assert_eq!(Solution::max_good_number(vec![2, 8, 16]), 1296);
}
