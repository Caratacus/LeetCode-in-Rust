// Tests for Problem 1797: Design Authentication Manager
// Java reference: src/test/java/g1701_1800/s1797_design_authentication_manager/SolutionTest.java

use leetcode_in_rust::s1797::design_authentication_manager::AuthenticationManager;

#[test]
fn test_authentication_manager() {
    let mut manager = AuthenticationManager::new(5);
    manager.renew("aaa".to_string(), 1);
    manager.generate("aaa".to_string(), 2);
    assert_eq!(manager.count_unexpired_tokens(6), 1);
}

#[test]
fn test_authentication_manager2() {
    let mut manager = AuthenticationManager::new(13);
    manager.generate("likt".to_string(), 1);
    manager.generate("kql".to_string(), 2);
    manager.generate("ybox".to_string(), 6);
    manager.renew("ybox".to_string(), 10);
    manager.renew("likt".to_string(), 15);
    assert_eq!(manager.count_unexpired_tokens(18), 2);
}
