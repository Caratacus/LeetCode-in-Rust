// Tests for Problem 3408: Design Task Manager
// Java reference: src/test/java/g3401_3500/s3408_design_task_manager/TaskManagerTest.java

use leetcode_in_rust::s3408::design_task_manager::TaskManager;

#[test]
fn test_test() {
    let mut task_manager = TaskManager::new(vec![vec![1, 101, 10], vec![2, 102, 20], vec![3, 103, 15]]);
    task_manager.add(4, 104, 5);
    task_manager.edit(102, 8);
    assert_eq!(task_manager.exec_top(), 3);
    task_manager.rmv(101);
    task_manager.add(5, 105, 15);
    assert_eq!(task_manager.exec_top(), 5);
}
