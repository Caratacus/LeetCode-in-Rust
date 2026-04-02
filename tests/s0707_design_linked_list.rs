// Tests for Problem 0707: Design Linked List
// Java reference: src/test/java/g0701_0800/s0707_design_linked_list/MyLinkedListTest.java

use leetcode_in_rust::s0707::design_linked_list::MyLinkedList;

#[test]
fn test_my_linked_list() {
    let mut list = MyLinkedList::new();
    list.add_at_head(1);
    list.add_at_tail(3);
    list.add_at_index(1, 2);
    assert_eq!(list.get(1), 2);
    list.delete_at_index(1);
    assert_eq!(list.get(1), 3);
}

#[test]
fn test_my_linked_list2() {
    let mut list = MyLinkedList::new();
    list.add_at_head(1);
    list.add_at_tail(3);
    list.add_at_index(1, 2);
    assert_eq!(list.get(1), 2);
    list.delete_at_index(0);
    assert_eq!(list.get(0), 2);
}

#[test]
fn test_my_linked_list3() {
    let mut list = MyLinkedList::new();
    list.add_at_head(1);
    list.add_at_tail(2);
    list.add_at_index(5, 3);
    assert_eq!(list.get(2), -1);
}

#[test]
fn test_my_linked_list4() {
    let mut list = MyLinkedList::new();
    list.add_at_index(0, 10);
    assert_eq!(list.get(0), 10);
}

#[test]
fn test_my_linked_list5() {
    let mut list = MyLinkedList::new();
    list.add_at_tail(5);
    assert_eq!(list.get(0), 5);
}

#[test]
fn test_my_linked_list6() {
    let mut list = MyLinkedList::new();
    list.add_at_head(1);
    list.delete_at_index(5);
    assert_eq!(list.get(0), 1);
}

#[test]
fn test_my_linked_list7() {
    let mut list = MyLinkedList::new();
    list.add_at_head(1);
    list.add_at_tail(2);
    list.add_at_tail(3);
    list.delete_at_index(0);
    assert_eq!(list.get(0), 2);
    list.delete_at_index(1);
    assert_eq!(list.get(1), -1);
}

#[test]
fn test_my_linked_list8() {
    let mut list = MyLinkedList::new();
    list.add_at_head(1);
    list.add_at_tail(2);
    list.add_at_tail(3);
    list.delete_at_index(1);
    assert_eq!(list.get(1), 3);
}

#[test]
fn test_my_linked_list9() {
    let mut list = MyLinkedList::new();
    list.add_at_head(1);
    list.add_at_tail(2);
    assert_eq!(list.get(5), -1);
}

#[test]
fn test_my_linked_list10() {
    let mut list = MyLinkedList::new();
    list.add_at_head(1);
    list.add_at_tail(2);
    list.add_at_index(1, 3);
    list.delete_at_index(0);
    list.add_at_tail(4);
    assert_eq!(list.get(0), 3);
    assert_eq!(list.get(1), 2);
    assert_eq!(list.get(2), 4);
}

#[test]
fn test_my_linked_list11() {
    let mut list = MyLinkedList::new();
    list.add_at_head(1);
    list.add_at_head(2);
    assert_eq!(list.get(0), 2);
    assert_eq!(list.get(1), 1);
}
