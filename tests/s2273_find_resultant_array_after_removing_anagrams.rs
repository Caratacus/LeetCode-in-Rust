// Tests for Problem 2273: Find Resultant Array After Removing Anagrams
// Java reference: src/test/java/g2201_2300/s2273_find_resultant_array_after_removing_anagrams/SolutionTest.java

use leetcode_in_rust::s2273::find_resultant_array_after_removing_anagrams::Solution;

#[test]
fn test_remove_anagrams() {
    assert_eq!(
        Solution::remove_anagrams(vec!["abba".to_string(), "baba".to_string(), "bbaa".to_string(), "cd".to_string(), "cd".to_string()]),
        vec!["abba".to_string(), "cd".to_string()]
    );
}

#[test]
fn test_remove_anagrams2() {
    assert_eq!(
        Solution::remove_anagrams(vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string()]),
        vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string()]
    );
}

#[test]
fn test_remove_anagrams3() {
    assert_eq!(Solution::remove_anagrams(vec![]), Vec::<String>::new());
}
