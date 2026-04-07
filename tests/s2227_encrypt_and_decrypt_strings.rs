// Tests for Problem 2227: Encrypt and Decrypt Strings
// Java reference: src/test/java/g2201_2300/s2227_encrypt_and_decrypt_strings/EncrypterTest.java

use leetcode_in_rust::s2227::encrypt_and_decrypt_strings::Encrypter;

#[test]
fn test_encrypter() {
    let mut encrypter = Encrypter::new(
        vec!['a', 'b', 'c', 'd'],
        vec!["ei".to_string(), "zf".to_string(), "ei".to_string(), "am".to_string()],
        vec!["abcd".to_string(), "acbd".to_string(), "adbc".to_string(), "badc".to_string(),
             "dacb".to_string(), "cadb".to_string(), "cbda".to_string(), "abad".to_string()],
    );
    assert_eq!(encrypter.encrypt("abcd".to_string()), "eizfeiam");
    assert_eq!(encrypter.decrypt("eizfeiam".to_string()), 2);
}
