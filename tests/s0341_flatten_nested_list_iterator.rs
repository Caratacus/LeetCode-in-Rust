// Tests for Problem 0341: Flatten Nested List Iterator
// Java reference: src/test/java/g0301_0400/s0341_flatten_nested_list_iterator/SolutionTest.java

// Note: This test requires NestedInteger support which is complex to construct.
// The test is commented out until proper NestedInteger construction utilities are available.

// use leetcode_in_rust::s0341::flatten_nested_list_iterator::NestedIterator;

#[test]
#[ignore = "Requires NestedInteger construction utilities"]
fn test_nested_iterator() {
    // TODO: Implement when NestedInteger utilities are available
    // Java: List<NestedInteger> list = Arrays.asList(
    //   new NestedInteger(Arrays.asList(new NestedInteger(1), new NestedInteger(1))),
    //   new NestedInteger(2),
    //   new NestedInteger(Arrays.asList(new NestedInteger(1), new NestedInteger(1))));
    // NestedIterator iterator = new NestedIterator(list);
    // List<Integer> result = new ArrayList<>();
    // while (iterator.hasNext()) { result.add(iterator.next()); }
    // assertThat(result, equalTo(Arrays.asList(1, 1, 2, 1, 1)));
}

#[test]
#[ignore = "Requires NestedInteger construction utilities"]
fn test_nested_iterator2() {
    // TODO: Implement when NestedInteger utilities are available
    // Java: NestedInteger integer1 = new NestedInteger(1);
    // NestedInteger integer2 = new NestedInteger(2);
    // NestedInteger integer3 = new NestedInteger(3);
    // List<NestedInteger> list = new ArrayList<>();
    // list.add(integer1);
    // list.add(new NestedInteger(Arrays.asList(integer2, integer3)));
    // NestedIterator iterator = new NestedIterator(list);
    // List<Integer> result = new ArrayList<>();
    // while (iterator.hasNext()) { result.add(iterator.next()); }
    // assertThat(result, equalTo(Arrays.asList(1, 2, 3)));
}
