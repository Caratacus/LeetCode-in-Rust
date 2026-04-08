// Tests for Problem 3216: Lexicographically Smallest String After a Swap
// Java reference: src/test/java/g3201_3300/s3216_lexicographically_smallest_string_after_a_swap/SolutionTest.java

use leetcode_in_rust::s3216::lexicographically_smallest_string_after_a_swap::Solution;

#[test]
fn test_get_smallest_string() {
    assert_eq!(Solution::get_smallest_string("45320".to_string()), "43520");
}

#[test]
fn test_get_smallest_string2() {
    assert_eq!(Solution::get_smallest_string("001".to_string()), "001");
}
