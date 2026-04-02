// Tests for Problem 0205: Isomorphic Strings
// Java reference: src/test/java/g0201_0300/s0205_isomorphic_strings/SolutionTest.java

use leetcode_in_rust::s0205::isomorphic_strings::Solution;

#[test]
fn test_is_isomorphic() {
    assert_eq!(Solution::is_isomorphic("egg".to_string(), "add".to_string()), true);
}

#[test]
fn test_is_isomorphic2() {
    assert_eq!(Solution::is_isomorphic("foo".to_string(), "bar".to_string()), false);
}

#[test]
fn test_is_isomorphic3() {
    assert_eq!(Solution::is_isomorphic("paper".to_string(), "title".to_string()), true);
}
