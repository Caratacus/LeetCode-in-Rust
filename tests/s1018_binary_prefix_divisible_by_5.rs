// Tests for Problem 1018: Binary Prefix Divisible By 5
// Java reference: src/test/java/g1001_1100/s1018_binary_prefix_divisible_by_5/SolutionTest.java

use leetcode_in_rust::s1018::binary_prefix_divisible_by_5::Solution;

#[test]
fn test_prefixes_div_by5() {
    assert_eq!(
        Solution::prefixes_div_by5(vec![0, 1, 1]),
        vec![true, false, false]
    );
}

#[test]
fn test_prefixes_div_by5_2() {
    assert_eq!(
        Solution::prefixes_div_by5(vec![1, 1, 1]),
        vec![false, false, false]
    );
}
