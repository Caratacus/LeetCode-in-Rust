// Tests for Problem 0211: Design Add and Search Words Data Structure
// Java reference: src/test/java/g0201_0300/s0211_design_add_and_search_words_data_structure/WordDictionaryTest.java

use leetcode_in_rust::s0211::design_add_and_search_words_data_structure::WordDictionary;

#[test]
fn test_word_dictionary() {
    let mut dictionary = WordDictionary::new();
    dictionary.add_word("bad".to_string());
    dictionary.add_word("dad".to_string());
    dictionary.add_word("mad".to_string());
    assert_eq!(dictionary.search("pad".to_string()), false);
    assert_eq!(dictionary.search("bad".to_string()), true);
    assert_eq!(dictionary.search(".ad".to_string()), true);
    assert_eq!(dictionary.search("b..".to_string()), true);
}
