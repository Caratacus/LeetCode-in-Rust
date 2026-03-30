// Problem 2642: design graph with shortest path calculator

pub struct Graph {
    n: i32,
    edges: Vec<Vec<i32>>,
}

impl Graph {
    pub fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        todo!()
    }

    pub fn add_edge(&mut self, edge: Vec<i32>) -> () {
        todo!()
    }

    pub fn shortest_path(&mut self, node1: i32, node2: i32) -> i32 {
        todo!()
    }

    pub fn get_key(&self) -> i32 {
        todo!()
    }

    pub fn get_value(&self) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void graphTest()
    //   Graph g =
    //   new Graph(
    //   4,
    //   CommonUtils.convertLeetCodeIrregularLengths2DArrayInputIntoJavaArray(
    //   "[0,2,5],[0,1,2],[1,2,1],[3,0,3]"));
    //   ... (4 more lines)
    #[test]
    fn test_graph_test() {
        // TODO: 翻译 Java 测试
    }
}
