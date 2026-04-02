// Tests for Problem 1160: Find Words That Can Be Formed by Characters
// Java reference: src/test/java/g1101_1200/s1160_find_words_that_can_be_formed_by_characters/SolutionTest.java

use leetcode_in_rust::s1160::find_words_that_can_be_formed_by_characters::Solution;

#[test]
fn test_count_characters() {
    assert_eq!(
        Solution::count_characters(
            vec!["cat".to_string(), "bt".to_string(), "hat".to_string(), "tree".to_string()],
            "atach".to_string()
        ),
        6
    );
}

#[test]
fn test_count_characters2() {
    assert_eq!(
        Solution::count_characters(
            vec!["hello".to_string(), "world".to_string(), "leetcode".to_string()],
            "welldonehoneyr".to_string()
        ),
        10
    );
}
