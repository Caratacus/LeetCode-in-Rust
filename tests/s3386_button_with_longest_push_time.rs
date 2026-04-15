// Tests for Problem 3386: Button With Longest Push Time
// Java reference: src/test/java/g3301_3400/s3386_button_with_longest_push_time/SolutionTest.java

use leetcode_in_rust::s3386::button_with_longest_push_time::Solution;

#[test]
fn test_button_with_longest_time() {
    assert_eq!(
        Solution::button_with_longest_time(vec![vec![1, 2], vec![2, 5], vec![3, 9], vec![1, 15]]),
        1
    );
}

#[test]
fn test_button_with_longest_time2() {
    assert_eq!(
        Solution::button_with_longest_time(vec![vec![10, 5], vec![1, 7]]),
        10
    );
}
