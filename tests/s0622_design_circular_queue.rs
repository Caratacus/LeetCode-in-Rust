// Tests for Problem 0622: Design Circular Queue
// Java reference: src/test/java/g0601_0700/s0622_design_circular_queue/MyCircularQueueTest.java

use leetcode_in_rust::s0622::design_circular_queue::MyCircularQueue;

#[test]
fn test_my_circular_queue() {
    let mut q = MyCircularQueue::new(3);
    assert_eq!(q.en_queue(1), true);
    assert_eq!(q.en_queue(2), true);
    assert_eq!(q.en_queue(3), true);
    assert_eq!(q.en_queue(4), false);
    assert_eq!(q.rear(), 3);
    assert_eq!(q.is_full(), true);
    assert_eq!(q.de_queue(), true);
    assert_eq!(q.en_queue(4), true);
    assert_eq!(q.rear(), 4);
}

#[test]
fn test_my_circular_queue3() {
    let mut q = MyCircularQueue::new(2);
    assert_eq!(q.front(), -1);
    assert_eq!(q.rear(), -1);
    assert_eq!(q.de_queue(), false);
    assert_eq!(q.is_empty(), true);
}

#[test]
fn test_my_circular_queue4() {
    let mut q = MyCircularQueue::new(1);
    assert_eq!(q.en_queue(10), true);
    assert_eq!(q.is_full(), true);
    assert_eq!(q.front(), 10);
    assert_eq!(q.rear(), 10);
    assert_eq!(q.de_queue(), true);
    assert_eq!(q.is_empty(), true);
    assert_eq!(q.front(), -1);
    assert_eq!(q.rear(), -1);
}

#[test]
fn test_my_circular_queue5() {
    let mut q = MyCircularQueue::new(3);

    assert_eq!(q.en_queue(1), true);
    assert_eq!(q.en_queue(2), true);
    assert_eq!(q.en_queue(3), true);
    assert_eq!(q.is_full(), true);
    assert_eq!(q.de_queue(), true);
    assert_eq!(q.de_queue(), true);
    assert_eq!(q.de_queue(), true);
    assert_eq!(q.is_empty(), true);
    assert_eq!(q.is_full(), false);
}

#[test]
fn test_my_circular_queue6() {
    let mut q = MyCircularQueue::new(2);

    assert_eq!(q.en_queue(5), true);
    assert_eq!(q.en_queue(6), true);
    assert_eq!(q.is_full(), true);

    assert_eq!(q.de_queue(), true);
    assert_eq!(q.de_queue(), true);
    assert_eq!(q.is_empty(), true);

    assert_eq!(q.en_queue(7), true);
    assert_eq!(q.front(), 7);
    assert_eq!(q.rear(), 7);
}
