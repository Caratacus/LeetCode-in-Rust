// Tests for Problem 3006: Find Beautiful Indices in the Given Array I
// Java reference: src/test/java/g3001_3100/s3006_find_beautiful_indices_in_the_given_array_i/SolutionTest.java

use leetcode_in_rust::s3006::find_beautiful_indices_in_the_given_array_i::Solution;

#[test]
fn test_beautiful_indices() {
    assert_eq!(
        Solution::beautiful_indices(
            "isawsquirrelnearmysquirrelhouseohmy".to_string(),
            "my".to_string(),
            "squirrel".to_string(),
            15
        ),
        vec![16, 33]
    );
}

#[test]
fn test_beautiful_indices2() {
    assert_eq!(
        Solution::beautiful_indices("abcd".to_string(), "a".to_string(), "a".to_string(), 4),
        vec![0]
    );
}
