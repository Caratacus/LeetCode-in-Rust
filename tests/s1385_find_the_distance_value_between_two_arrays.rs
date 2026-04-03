// Tests for Problem 1385: Find the Distance Value Between Two Arrays
// Java reference: src/test/java/g1301_1400/s1385_find_the_distance_value_between_two_arrays/SolutionTest.java

use leetcode_in_rust::s1385::find_the_distance_value_between_two_arrays::Solution;

#[test]
fn test_find_the_distance_value() {
    assert_eq!(
        Solution::find_the_distance_value(vec![4, 5, 8], vec![10, 9, 1, 8], 2),
        2
    );
}

#[test]
fn test_find_the_distance_value2() {
    assert_eq!(
        Solution::find_the_distance_value(vec![1, 4, 2, 3], vec![-4, -3, 6, 10, 20, 30], 3),
        2
    );
}

#[test]
fn test_find_the_distance_value3() {
    assert_eq!(
        Solution::find_the_distance_value(vec![2, 1, 100, 3], vec![-5, -2, 10, -3, 7], 6),
        1
    );
}
