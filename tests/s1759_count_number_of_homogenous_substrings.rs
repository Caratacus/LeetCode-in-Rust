// Tests for Problem 1759: Count Number of Homogenous Substrings
// Java reference: src/test/java/g1701_1800/s1759_count_number_of_homogenous_substrings/SolutionTest.java

use leetcode_in_rust::s1759::count_number_of_homogenous_substrings::Solution;

#[test]
fn test_count_homogenous() {
    assert_eq!(
        Solution::count_homogenous("abbcccaa".to_string()),
        13
    );
}

#[test]
fn test_count_homogenous2() {
    assert_eq!(Solution::count_homogenous("xy".to_string()), 2);
}

#[test]
fn test_count_homogenous3() {
    assert_eq!(Solution::count_homogenous("zzzzz".to_string()), 15);
}
