// Tests for Problem 2433: Find the Original Array of Prefix Xor
// Java reference: src/test/java/g2401_2500/s2433_find_the_original_array_of_prefix_xor/SolutionTest.java

use leetcode_in_rust::s2433::find_the_original_array_of_prefix_xor::Solution;

#[test]
fn test_find_array() {
    assert_eq!(
        Solution::find_array(vec![5, 2, 0, 3, 1]),
        vec![5, 7, 2, 3, 2]
    );
}

#[test]
fn test_find_array2() {
    assert_eq!(Solution::find_array(vec![13]), vec![13]);
}
