// Tests for Problem 0895: Maximum Frequency Stack
// Java reference: src/test/java/g0801_0900/s0895_maximum_frequency_stack/SolutionTest.java

use leetcode_in_rust::s0895::maximum_frequency_stack::FreqStack;

#[test]
fn test_freq_stack() {
    let mut freq_stack = FreqStack::new();
    freq_stack.push(5);
    freq_stack.push(7);
    freq_stack.push(5);
    freq_stack.push(7);
    freq_stack.push(4);
    freq_stack.push(5);
    assert_eq!(freq_stack.pop(), 5);
    assert_eq!(freq_stack.pop(), 7);
    assert_eq!(freq_stack.pop(), 5);
    assert_eq!(freq_stack.pop(), 4);
}
