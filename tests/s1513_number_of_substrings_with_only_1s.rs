// Tests for Problem 1513: Number of Substrings With Only 1s
// Java reference: src/test/java/g1501_1600/s1513_number_of_substrings_with_only_1s/SolutionTest.java

use leetcode_in_rust::s1513::number_of_substrings_with_only_1s::Solution;

#[test]
fn test_num_sub() {
    assert_eq!(Solution::num_sub("0110111".to_string()), 9);
}

#[test]
fn test_num_sub2() {
    assert_eq!(Solution::num_sub("101".to_string()), 2);
}

#[test]
fn test_num_sub3() {
    assert_eq!(Solution::num_sub("111111".to_string()), 21);
}
