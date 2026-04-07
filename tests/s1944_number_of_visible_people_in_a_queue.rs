// Tests for Problem 1944: Number of Visible People in a Queue
// Java reference: src/test/java/g1901_2000/s1944_number_of_visible_people_in_a_queue/SolutionTest.java

use leetcode_in_rust::s1944::number_of_visible_people_in_a_queue::Solution;

#[test]
fn test_can_see_persons_count() {
    assert_eq!(
        Solution::can_see_persons_count(vec![10, 6, 8, 5, 11, 9]),
        vec![3, 1, 2, 1, 1, 0]
    );
}

#[test]
fn test_can_see_persons_count2() {
    assert_eq!(
        Solution::can_see_persons_count(vec![5, 1, 2, 3, 10]),
        vec![4, 1, 1, 1, 0]
    );
}
