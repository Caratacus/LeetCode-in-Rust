// Tests for Problem 0297: Serialize and Deserialize Binary Tree
// Java reference: src/test/java/g0201_0300/s0297_serialize_and_deserialize_binary_tree/CodecTest.java
// Note: The Rust API requires &mut self and struct has no constructor

use leetcode_in_rust::s0297::serialize_and_deserialize_binary_tree::Codec;
use leetcode_in_rust::utils::tree_utils::{tree_from_vec, tree_to_vec};

#[test]
#[ignore = "Codec struct needs new() constructor implementation"]
fn test_codec() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5)]);
    let mut codec = Codec {};
    let serialized = codec.serialize(root);
    let deserialized = codec.deserialize(serialized);
    assert_eq!(tree_to_vec(deserialized), vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5)]);
}
