// Tests for Problem 0559: Maximum Depth of N-ary Tree
// Java reference: src/test/java/g0501_0600/s0559_maximum_depth_of_n_ary_tree/SolutionTest.java

use leetcode_in_rust::s0559::maximum_depth_of_n_ary_tree::Solution;
use leetcode_in_rust::common::graph_node::GraphNode;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_max_depth() {
    // Build tree: 1 -> [3, 2, 4], 3 -> [5, 6]
    let node5 = Rc::new(RefCell::new(GraphNode::new(5)));
    let node6 = Rc::new(RefCell::new(GraphNode::new(6)));

    let mut node3 = GraphNode::new(3);
    node3.neighbors = vec![node5, node6];

    let node2 = GraphNode::new(2);
    let node4 = GraphNode::new(4);

    let mut root = GraphNode::new(1);
    root.neighbors = vec![
        Rc::new(RefCell::new(node3)),
        Rc::new(RefCell::new(node2)),
        Rc::new(RefCell::new(node4)),
    ];

    assert_eq!(Solution::max_depth(Some(Rc::new(RefCell::new(root)))), 3);
}

#[test]
fn test_max_depth_empty() {
    assert_eq!(Solution::max_depth(None), 0);
}
