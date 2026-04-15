// Tests for Problem 3469: Find Minimum Cost to Remove Array Elements
// Java reference: src/test/java/g3401_3500/s3469_find_minimum_cost_to_remove_array_elements/SolutionTest.java

use leetcode_in_rust::s3469::find_minimum_cost_to_remove_array_elements::Solution;

#[test]
fn test_min_cost() {
    assert_eq!(Solution::min_cost(vec![6, 2, 8, 4]), 12);
}

#[test]
fn test_min_cost2() {
    assert_eq!(Solution::min_cost(vec![2, 1, 3, 3]), 5);
}

#[test]
fn test_min_cost3() {
    assert_eq!(Solution::min_cost(vec![83, 47, 66, 24, 57, 85, 16]), 224);
}
