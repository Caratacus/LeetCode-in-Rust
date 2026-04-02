// Tests for Problem 0155: Min Stack
// Java reference: src/test/java/g0121_0200/s0155_min_stack/SolutionTest.java

use leetcode_in_rust::s0155::min_stack::MinStack;

#[test]
fn test_min_stack() {
    let mut stack = MinStack::new();
    stack.push(-2);
    stack.push(0);
    stack.push(-3);
    assert_eq!(stack.get_min(), -3);
    stack.pop();
    assert_eq!(stack.top(), 0);
    assert_eq!(stack.get_min(), -2);
}
