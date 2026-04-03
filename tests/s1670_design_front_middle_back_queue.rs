// Tests for Problem 1670: Design Front Middle Back Queue
// Java reference: src/test/java/g1601_1700/s1670_design_front_middle_back_queue/SolutionTest.java

use leetcode_in_rust::s1670::design_front_middle_back_queue::FrontMiddleBackQueue;

#[test]
fn test_front_middle_back_queue() {
    let mut q = FrontMiddleBackQueue::new();
    q.push_front(1);
    q.push_back(2);
    q.push_middle(3);
    q.push_middle(4);
    assert_eq!(q.pop_front(), 1);
    assert_eq!(q.pop_middle(), 3);
    assert_eq!(q.pop_middle(), 4);
    assert_eq!(q.pop_back(), 2);
    assert_eq!(q.pop_front(), -1);
}
