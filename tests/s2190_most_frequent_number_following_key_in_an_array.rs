// Tests for Problem 2190: Most Frequent Number Following Key in an Array
// Java reference: src/test/java/g2101_2200/s2190_most_frequent_number_following_key_in_an_array/SolutionTest.java

use leetcode_in_rust::s2190::most_frequent_number_following_key_in_an_array::Solution;

#[test]
fn test_most_frequent() {
    assert_eq!(Solution::most_frequent(vec![1, 100, 200, 1, 100], 1), 100);
}

#[test]
fn test_most_frequent2() {
    assert_eq!(Solution::most_frequent(vec![2, 2, 2, 2, 3], 2), 2);
}
