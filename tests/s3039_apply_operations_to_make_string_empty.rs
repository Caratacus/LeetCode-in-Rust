// Tests for Problem 3039: Apply Operations to Make String Empty
// Java reference: src/test/java/g3001_3100/s3039_apply_operations_to_make_string_empty/SolutionTest.java

use leetcode_in_rust::s3039::apply_operations_to_make_string_empty::Solution;

#[test]
fn test_last_non_empty_string() {
    assert_eq!(
        Solution::last_non_empty_string(String::from("aabcbbca")),
        String::from("ba")
    );
}
