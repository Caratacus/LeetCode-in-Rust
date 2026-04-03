// Tests for Problem 1353: Maximum Number of Events That Can Be Attended
// Java reference: src/test/java/g1301_1400/s1353_maximum_number_of_events_that_can_be_attended/SolutionTest.java

use leetcode_in_rust::s1353::maximum_number_of_events_that_can_be_attended::Solution;

#[test]
fn test_max_events() {
    let result = Solution::max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4]]);
    assert_eq!(result, 3);
}

#[test]
fn test_max_events2() {
    let result = Solution::max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 2]]);
    assert_eq!(result, 4);
}
