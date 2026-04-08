// Tests for Problem 3343: Count Number of Balanced Permutations
// Java reference: src/test/java/g3301_3400/s3343_count_number_of_balanced_permutations/SolutionTest.java

use leetcode_in_rust::s3343::count_number_of_balanced_permutations::Solution;

#[test]
fn test_count_balanced_permutations() {
    assert_eq!(Solution::count_balanced_permutations("123".to_string()), 2);
}

#[test]
fn test_count_balanced_permutations2() {
    assert_eq!(Solution::count_balanced_permutations("112".to_string()), 1);
}

#[test]
fn test_count_balanced_permutations3() {
    assert_eq!(Solution::count_balanced_permutations("12345".to_string()), 0);
}
