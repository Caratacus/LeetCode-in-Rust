// Tests for Problem 2266: Count Number of Texts
// Java reference: src/test/java/g2201_2300/s2266_count_number_of_texts/SolutionTest.java

use leetcode_in_rust::s2266::count_number_of_texts::Solution;

#[test]
fn test_count_texts() {
    assert_eq!(Solution::count_texts("22233".to_string()), 8);
}

#[test]
fn test_count_texts2() {
    assert_eq!(Solution::count_texts("222222222222222222222222222222222222".to_string()), 82876089);
}
