// Tests for Problem 2150: Find All Lonely Numbers in the Array
// Java reference: src/test/java/g2101_2200/s2150_find_all_lonely_numbers_in_the_array/SolutionTest.java

use leetcode_in_rust::s2150::find_all_lonely_numbers_in_the_array::Solution;

#[test]
fn test_find_lonely() {
    let mut result = Solution::find_lonely(vec![10, 6, 5, 8]);
    result.sort();
    assert_eq!(result, vec![8, 10]);
}

#[test]
fn test_find_lonely2() {
    let mut result = Solution::find_lonely(vec![1, 3, 5, 3]);
    result.sort();
    assert_eq!(result, vec![1, 5]);
}
