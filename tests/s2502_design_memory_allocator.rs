// Tests for Problem 2502: Design Memory Allocator
// Java reference: src/test/java/g2401_2500/s2502_design_memory_allocator/SolutionTest.java

use leetcode_in_rust::s2502::design_memory_allocator::Allocator;

#[test]
fn test_allocator() {
    let mut loc = Allocator::new(10);
    // The leftmost block's first index is 0. The memory array becomes [1,_,_,_,_,_,_,_,_,_].
    assert_eq!(loc.allocate(1, 1), 0);
    // The leftmost block's first index is 1. The memory array becomes [1,2,_,_,_,_,_,_,_,_].
    assert_eq!(loc.allocate(1, 2), 1);
    // The leftmost block's first index is 2. The memory array becomes [1,2,3,_,_,_,_,_,_,_].
    assert_eq!(loc.allocate(1, 3), 2);
    // Free all memory blocks with id 2. The memory array becomes [1,_,3,_,_,_,_,_,_,_].
    assert_eq!(loc.free(2), 1);
    // The leftmost block's first index is 1. The memory array becomes [1,4,3,_,_,_,_,_,_,_].
    assert_eq!(loc.allocate(1, 4), 1);
    // The leftmost block's first index is 3. The memory array becomes [1,4,3,4,_,_,_,_,_,_].
    assert_eq!(loc.allocate(1, 4), 3);
    // Free all memory blocks with id 4. The memory array becomes [1,_,3,_,_,_,_,_,_,_].
    assert_eq!(loc.free(4), 2);
    // The leftmost block's first index is 1. The memory array becomes [1,5,3,_,_,_,_,_,_,_].
    assert_eq!(loc.allocate(1, 5), 1);
    // The leftmost block's first index is 3. The memory array becomes [1,5,3,5,_,_,_,_,_,_].
    assert_eq!(loc.allocate(1, 5), 3);
    // Free all memory blocks with id 5. The memory array becomes [1,_,3,_,_,_,_,_,_,_].
    assert_eq!(loc.free(5), 2);
    // The leftmost block's first index is 1. The memory array becomes [1,6,3,_,_,_,_,_,_,_].
    assert_eq!(loc.allocate(1, 6), 1);
    // The leftmost block's first index is 3. The memory array becomes [1,6,3,6,_,_,_,_,_,_].
    assert_eq!(loc.allocate(1, 6), 3);
    // Free all memory blocks with id 6. The memory array becomes [1,_,3,_,_,_,_,_,_,_].
    assert_eq!(loc.free(6), 2);
    // The leftmost block's first index is 1. The memory array becomes [1,7,3,_,_,_,_,_,_,_].
    assert_eq!(loc.allocate(1, 7), 1);
    // The leftmost block's first index is 3. The memory array becomes [1,7,3,7,_,_,_,_,_,_].
    assert_eq!(loc.allocate(1, 7), 3);
    // Free all memory blocks with id 7. The memory array becomes [1,_,3,_,_,_,_,_,_,_].
    assert_eq!(loc.free(7), 2);
    // The leftmost block's first index is 1. The memory array becomes [1,8,3,_,_,_,_,_,_,_].
    assert_eq!(loc.allocate(1, 8), 1);
    // The leftmost block's first index is 3. The memory array becomes [1,8,3,8,_,_,_,_,_,_].
    assert_eq!(loc.allocate(1, 8), 3);
    // Free all memory blocks with id 8. The memory array becomes [1,_,3,_,_,_,_,_,_,_].
    assert_eq!(loc.free(8), 2);
    // The leftmost block's first index is 1. The memory array becomes [1,9,3,_,_,_,_,_,_,_].
    assert_eq!(loc.allocate(1, 9), 1);
    // The leftmost block's first index is 3. The memory array becomes [1,9,3,9,_,_,_,_,_,_].
    assert_eq!(loc.allocate(1, 9), 3);
    // Free all memory blocks with id 9. The memory array becomes [1,_,3,_,_,_,_,_,_,_].
    assert_eq!(loc.free(9), 2);
    // The leftmost block's first index is 1. The memory array becomes [1,10,3,_,_,_,_,_,_,_].
    assert_eq!(loc.allocate(1, 10), 1);
    // The leftmost block's first index is 3. The memory array becomes [1,10,3,10,_,_,_,_,_,_].
    assert_eq!(loc.allocate(1, 10), 3);
    // Free all memory blocks with id 10. The memory array becomes [1,_,3,_,_,_,_,_,_,_].
    assert_eq!(loc.free(10), 2);
    // The leftmost block's first index is 1. The memory array becomes [1,11,3,_,_,_,_,_,_,_].
    assert_eq!(loc.allocate(1, 11), 1);
    // The leftmost block's first index is 3. The memory array becomes [1,11,3,11,_,_,_,_,_,_].
    assert_eq!(loc.allocate(1, 11), 3);
    // Free all memory blocks with id 11. The memory array becomes [1,_,3,_,_,_,_,_,_,_].
    assert_eq!(loc.free(11), 2);
}
