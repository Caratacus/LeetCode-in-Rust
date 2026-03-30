// Problem 1157: online majority element in subarray

pub struct MajorityChecker {
    arr: Vec<i32>,
}

impl MajorityChecker {
    pub fn new(arr: Vec<i32>) -> Self {
        todo!()
    }

    pub fn query(&mut self, left: i32, right: i32, threshold: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void majorityCheckerTest()
    //   MajorityChecker majorityChecker = new MajorityChecker(new int[] {1, 1, 2, 2, 1, 1});
    //   assertThat(majorityChecker.query(0, 5, 4), equalTo(1));
    //   assertThat(majorityChecker.query(0, 3, 3), equalTo(-1));
    //   assertThat(majorityChecker.query(2, 3, 2), equalTo(2));
    #[test]
    fn test_majority_checker_test() {
        // TODO: 翻译 Java 测试
    }
}
