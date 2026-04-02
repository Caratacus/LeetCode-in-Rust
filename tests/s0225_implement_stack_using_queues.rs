// Tests for Problem 0225: Implement Stack using Queues
// Java reference: src/test/java/g0201_0300/s0225_implement_stack_using_queues/MyStackTest.java

use leetcode_in_rust::s0225::implement_stack_using_queues::MyStack;

#[test]
fn test_my_stack() {
    let mut stack = MyStack::new();
    stack.push(1);
    stack.push(2);
    assert_eq!(stack.top(), 2);
    assert_eq!(stack.pop(), 2);
    assert_eq!(stack.empty(), false);
}
