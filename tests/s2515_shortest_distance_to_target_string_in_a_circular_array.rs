// Tests for Problem 2515: Shortest Distance to Target String in a Circular Array
// Java reference: src/test/java/g2401_2500/s2515_shortest_distance_to_target_string_in_a_circular_array/SolutionTest.java

use leetcode_in_rust::s2515::shortest_distance_to_target_string_in_a_circular_array::Solution;

#[test]
fn test_closet_target() {
    assert_eq!(
        Solution::closet_target(
            vec!["hello".to_string(), "i".to_string(), "am".to_string(), "leetcode".to_string(), "hello".to_string()],
            "hello".to_string(),
            1
        ),
        1
    );
}

#[test]
fn test_closet_target2() {
    assert_eq!(
        Solution::closet_target(
            vec!["a".to_string(), "b".to_string(), "leetcode".to_string()],
            "leetcode".to_string(),
            0
        ),
        1
    );
}

#[test]
fn test_closet_target3() {
    assert_eq!(
        Solution::closet_target(
            vec!["i".to_string(), "eat".to_string(), "leetcode".to_string()],
            "ate".to_string(),
            0
        ),
        -1
    );
}
