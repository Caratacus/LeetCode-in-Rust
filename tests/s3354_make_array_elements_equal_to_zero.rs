// Tests for Problem 3354: Make Array Elements Equal to Zero
// Java reference: src/test/java/g3301_3400/s3354_make_array_elements_equal_to_zero/SolutionTest.java

use leetcode_in_rust::s3354::make_array_elements_equal_to_zero::Solution;

#[test]
fn test_count_valid_selections() {
    assert_eq!(Solution::count_valid_selections(vec![1, 0, 2, 0, 3]), 2);
}

#[test]
fn test_count_valid_selections2() {
    assert_eq!(Solution::count_valid_selections(vec![2, 3, 4, 0, 4, 1, 0]), 0);
}

#[test]
fn test_count_valid_selections3() {
    assert_eq!(
        Solution::count_valid_selections(vec![16, 13, 10, 0, 0, 0, 10, 6, 7, 8, 7]),
        3
    );
}
