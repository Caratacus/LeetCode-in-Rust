// Tests for Problem 2404: Most Frequent Even Element
// Java reference: src/test/java/g2401_2500/s2404_most_frequent_even_element/SolutionTest.java

use leetcode_in_rust::s2404::most_frequent_even_element::Solution;

#[test]
fn test_most_frequent_even() {
    assert_eq!(Solution::most_frequent_even(vec![0, 1, 2, 2, 4, 4, 1]), 2);
}

#[test]
fn test_most_frequent_even2() {
    assert_eq!(Solution::most_frequent_even(vec![4, 4, 4, 9, 2, 4]), 4);
}

#[test]
fn test_most_frequent_even3() {
    assert_eq!(Solution::most_frequent_even(vec![29, 47, 21, 41, 13, 37, 25, 7]), -1);
}
