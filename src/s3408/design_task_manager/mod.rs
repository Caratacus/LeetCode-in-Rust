// Problem 3408: design task manager

pub struct TaskManager {
    tasks: Vec<Vec<i32>>,
}

impl TaskManager {
    pub fn new(tasks: Vec<Vec<i32>>) -> Self {
        todo!()
    }

    pub fn add(&mut self, user_id: i32, task_id: i32, priority: i32) -> () {
        todo!()
    }

    pub fn edit(&mut self, task_id: i32, new_priority: i32) -> () {
        todo!()
    }

    pub fn rmv(&mut self, task_id: i32) -> () {
        todo!()
    }

    pub fn exec_top(&self) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void test()
    //   // Initializes with three tasks for Users 1, 2, and 3.
    //   TaskManager taskManager =
    //   new TaskManager(
    //   List.of(List.of(1, 101, 10), List.of(2, 102, 20), List.of(3, 103, 15)));
    //   // Adds task 104 with priority 5 for User 4.
    //   ... (11 more lines)
    #[test]
    fn test_test() {
        // TODO: 翻译 Java 测试
    }
}
