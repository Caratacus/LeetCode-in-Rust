// Tests for Problem 0208: Implement Trie (Prefix Tree)
// Java reference: src/test/java/g0201_0300/s0208_implement_trie_prefix_tree/TrieTest.java

use leetcode_in_rust::s0208::implement_trie_prefix_tree::Trie;

#[test]
fn test_trie() {
    let mut trie = Trie::new();
    trie.insert("apple".to_string());
    // return True
    assert_eq!(trie.search("apple".to_string()), true);
    // return False
    assert_eq!(trie.search("app".to_string()), false);
    // return True
    assert_eq!(trie.starts_with("app".to_string()), true);
    trie.insert("app".to_string());
    // return True
    assert_eq!(trie.search("app".to_string()), true);
}
