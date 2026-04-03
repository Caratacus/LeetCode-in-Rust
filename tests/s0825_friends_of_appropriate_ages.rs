// Tests for Problem 0825: Friends Of Appropriate Ages
// Java reference: src/test/java/g0801_0900/s0825_friends_of_appropriate_ages/SolutionTest.java

use leetcode_in_rust::s0825::friends_of_appropriate_ages::Solution;

#[test]
fn test_num_friend_requests() {
    assert_eq!(Solution::num_friend_requests(vec![16, 16]), 2);
}

#[test]
fn test_num_friend_requests2() {
    assert_eq!(Solution::num_friend_requests(vec![16, 17, 18]), 2);
}

#[test]
fn test_num_friend_requests3() {
    assert_eq!(Solution::num_friend_requests(vec![20, 30, 100, 110, 120]), 3);
}
