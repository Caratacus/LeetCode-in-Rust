// Problem 0690: employee importance

use crate::common::employee::Employee;

pub struct Solution;

impl Solution {
    pub fn get_importance(employees: Vec<Employee>, id: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void getImportance()
    //   List<Employee> employees =
    //   Arrays.asList(
    //   new Employee(1, 5, Arrays.asList(2, 3)),
    //   new Employee(2, 3, Arrays.asList()),
    //   new Employee(3, 3, Arrays.asList()));
    //   ... (1 more lines)
    #[test]
    fn test_get_importance() {
        // TODO: 翻译 Java 测试
    }

    // Java: void getImportance2()
    //   List<Employee> employees =
    //   Arrays.asList(
    //   new Employee(1, 5, Arrays.asList(2, 3)),
    //   new Employee(2, 3, Arrays.asList(4)),
    //   new Employee(3, 4, Arrays.asList()),
    //   ... (2 more lines)
    #[test]
    fn test_get_importance2() {
        // TODO: 翻译 Java 测试
    }
}
