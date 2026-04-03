// Tests for Problem 1389: Create Target Array in the Given Order
// Java reference: src/test/java/g1301_1400/s1389_create_target_array_in_the_given_order/SolutionTest.java

use leetcode_in_rust::s1389::create_target_array_in_the_given_order::Solution;

#[test]
fn test_create_target_array() {
    assert_eq!(
        Solution::create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]),
        vec![0, 4, 1, 3, 2]
    );
}

#[test]
fn test_create_target_array2() {
    assert_eq!(
        Solution::create_target_array(vec![1, 2, 3, 4, 0], vec![0, 1, 2, 3, 0]),
        vec![0, 1, 2, 3, 4]
    );
}
