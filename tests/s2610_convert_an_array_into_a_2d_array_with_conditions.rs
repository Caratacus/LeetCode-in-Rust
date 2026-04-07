// Tests for Problem 2610: Convert an Array Into a 2D Array With Conditions
// Java reference: src/test/java/g2601_2700/s2610_convert_an_array_into_a_2d_array_with_conditions/SolutionTest.java

use leetcode_in_rust::s2610::convert_an_array_into_a_2d_array_with_conditions::Solution;

#[test]
fn test_find_matrix() {
    let result = Solution::find_matrix(vec![1, 3, 4, 1, 2, 3, 1]);
    // Result should contain rows [1,2,3,4], [1,3], [1] in some order
    assert_eq!(result.len(), 3);
}

#[test]
fn test_find_matrix2() {
    let result = Solution::find_matrix(vec![1, 2, 3, 4]);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].len(), 4);
}
