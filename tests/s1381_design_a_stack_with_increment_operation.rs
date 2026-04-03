// Tests for Problem 1381: Design a Stack With Increment Operation
// Java reference: src/test/java/g1301_1400/s1381_design_a_stack_with_increment_operation/SolutionTest.java

use leetcode_in_rust::s1381::design_a_stack_with_increment_operation::CustomStack;

#[test]
fn test_custom_stack() {
    // CustomStack customStack = new CustomStack(3);
    let mut stack = CustomStack::new(3);

    // stack becomes [1]
    stack.push(1);

    // stack becomes [1, 2]
    stack.push(2);

    // return 2, stack becomes [1]
    assert_eq!(stack.pop(), 2);

    // stack becomes [1, 2]
    stack.push(2);

    // stack becomes [1, 2, 3]
    stack.push(3);

    // stack still [1, 2, 3], max_size reached
    stack.push(4);

    // stack becomes [2, 3, 4]
    stack.increment(5, 100);

    // stack becomes [102, 103, 4]
    stack.increment(2, 100);

    // return 4, stack becomes [102, 103]
    assert_eq!(stack.pop(), 4);

    // return 103, stack becomes [102]
    assert_eq!(stack.pop(), 103);

    // return 102, stack becomes []
    assert_eq!(stack.pop(), 102);

    // return -1, stack is empty
    assert_eq!(stack.pop(), -1);
}
