// Tests for Problem 0535: Encode and Decode TinyURL
// Java reference: src/test/java/g0501_0600/s0535_encode_and_decode_tinyurl/SolutionTest.java

use leetcode_in_rust::s0535::encode_and_decode_tinyurl::Codec;

#[test]
fn test_codec() {
    let mut codec = Codec {};
    let url = "https://leetcode.com/problems/design-tinyurl";
    let tiny = codec.encode(url.to_string());
    let ans = codec.decode(tiny);
    assert_eq!(ans, url);
}
