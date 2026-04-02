// Tests for Problem 0232: Implement Queue using Stacks
// Java reference: src/test/java/g0201_0300/s0232_implement_queue_using_stacks/MyQueueTest.java

use leetcode_in_rust::s0232::implement_queue_using_stacks::MyQueue;

#[test]
fn test_my_queue() {
    let mut queue = MyQueue::new();
    queue.push(1);
    queue.push(2);
    assert_eq!(queue.peek(), 1);
    assert_eq!(queue.pop(), 1);
    assert_eq!(queue.empty(), false);
}
