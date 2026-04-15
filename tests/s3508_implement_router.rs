// Tests for Problem 3508: Implement Router
// Java reference: src/test/java/g3501_3600/s3508_implement_router/RouterTest.java

use leetcode_in_rust::s3508::implement_router::Router;

#[test]
fn test_router() {
    let mut router = Router::new(3);
    assert_eq!(router.add_packet(1, 4, 90), true);
    assert_eq!(router.add_packet(2, 5, 90), true);
    assert_eq!(router.add_packet(1, 4, 90), false);
    assert_eq!(router.add_packet(3, 5, 95), true);
    assert_eq!(router.add_packet(4, 5, 105), true);
    assert_eq!(router.forward_packet(), vec![2, 5, 90]);
    assert_eq!(router.add_packet(5, 2, 110), true);
    assert_eq!(router.get_count(5, 100, 110), 1);
}

#[test]
fn test_router2() {
    let mut router = Router::new(2);
    assert_eq!(router.add_packet(7, 4, 90), true);
    assert_eq!(router.forward_packet(), vec![7, 4, 90]);
    assert_eq!(router.forward_packet(), vec![]);
}

#[test]
fn test_router3() {
    let mut router = Router::new(3);
    assert_eq!(router.add_packet(1, 4, 6), true);
    assert_eq!(router.get_count(4, 1, 4), 0);
}

#[test]
fn test_router4() {
    let mut router = Router::new(2);
    assert_eq!(router.add_packet(2, 5, 1), true);
    assert_eq!(router.forward_packet(), vec![2, 5, 1]);
    assert_eq!(router.get_count(5, 1, 1), 0);
}
