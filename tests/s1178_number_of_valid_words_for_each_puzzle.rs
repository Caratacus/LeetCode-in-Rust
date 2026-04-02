// Tests for Problem 1178: Number of Valid Words for Each Puzzle
// Java reference: src/test/java/g1101_1200/s1178_number_of_valid_words_for_each_puzzle/SolutionTest.java

use leetcode_in_rust::s1178::number_of_valid_words_for_each_puzzle::Solution;

#[test]
fn test_find_num_of_valid_words() {
    assert_eq!(
        Solution::find_num_of_valid_words(
            vec![
                "aaaa".to_string(),
                "asas".to_string(),
                "able".to_string(),
                "ability".to_string(),
                "actt".to_string(),
                "actor".to_string(),
                "access".to_string()
            ],
            vec![
                "aboveyz".to_string(),
                "abrodyz".to_string(),
                "abslute".to_string(),
                "absoryz".to_string(),
                "actresz".to_string(),
                "gaswxyz".to_string()
            ]
        ),
        vec![1, 1, 3, 2, 4, 0]
    );
}

#[test]
fn test_find_num_of_valid_words2() {
    assert_eq!(
        Solution::find_num_of_valid_words(
            vec!["apple".to_string(), "pleas".to_string(), "please".to_string()],
            vec![
                "aelwxyz".to_string(),
                "aelpxyz".to_string(),
                "aelpsxy".to_string(),
                "saelpxy".to_string(),
                "xaelpsy".to_string()
            ]
        ),
        vec![0, 1, 3, 2, 0]
    );
}
