// Tests for Problem 0385: Mini Parser
// Java reference: src/test/java/g0301_0400/s0385_mini_parser/SolutionTest.java

use leetcode_in_rust::s0385::mini_parser::Solution;

#[test]
fn test_deserialize() {
    // "324" should return a NestedInteger with integer value 324
    let result = Solution::deserialize("324".to_string());
    assert_eq!(result.get_integer(), Some(324));
}

#[test]
fn test_deserialize2() {
    // "[123,[456,[789]]]" should return a nested list
    let mut result = Solution::deserialize("[123,[456,[789]]]".to_string());
    let list = result.get_list().expect("should be a list");
    assert_eq!(list.len(), 2);
    assert_eq!(list[0].get_integer(), Some(123));
    let inner_list = &mut list[1];
    let inner = inner_list.get_list().expect("should be a list");
    assert_eq!(inner.len(), 2);
    assert_eq!(inner[0].get_integer(), Some(456));
}
