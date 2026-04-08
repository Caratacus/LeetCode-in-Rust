// Tests for Problem 3152: Special Array II
// Java reference: src/test/java/g3101_3200/s3152_special_array_ii/SolutionTest.java

use leetcode_in_rust::s3152::special_array_ii::Solution;

#[test]
fn test_is_array_special() {
    assert_eq!(
        Solution::is_array_special(vec![3, 4, 1, 2, 6], vec![vec![0, 4]]),
        vec![false]
    );
}
