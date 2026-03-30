// Problem 0895: maximum frequency stack
use std::cell::RefCell;
use std::rc::Rc;

use crate::common::graph_node::GraphNode;

pub struct FreqStack {}

impl FreqStack {
    pub fn new() -> Self {
        todo!()
    }

    pub fn add_node(&mut self, x: i32) -> () {
        todo!()
    }

    pub fn remove_node(&mut self) -> Option<Rc<RefCell<GraphNode>>> {
        todo!()
    }

    pub fn push(&mut self, val: i32) -> () {
        todo!()
    }

    pub fn pop(&mut self) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void freqStack()
    //   FreqStack freqStack = new FreqStack();
    //   // The stack is [5]
    //   freqStack.push(5);
    //   // The stack is [5,7]
    //   freqStack.push(7);
    //   ... (17 more lines)
    #[test]
    fn test_freq_stack() {
        // TODO: 翻译 Java 测试
    }
}
