// Tests for Problem 1838: Frequency of the Most Frequent Element
// Java reference: src/test/java/g1801_1900/s1838_frequency_of_the_most_frequent_element/SolutionTest.java

use leetcode_in_rust::s1838::frequency_of_the_most_frequent_element::Solution;

#[test]
fn test_max_frequency() {
    assert_eq!(Solution::max_frequency(vec![1, 2, 4], 5), 3);
}

#[test]
fn test_max_frequency2() {
    assert_eq!(Solution::max_frequency(vec![1, 4, 8, 13], 5), 2);
}

#[test]
fn test_max_frequency3() {
    assert_eq!(Solution::max_frequency(vec![3, 9, 6], 2), 1);
}
