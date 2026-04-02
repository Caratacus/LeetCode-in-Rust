// Problem 0384: shuffle an array

pub struct Solution {
    original: Vec<i32>,
}

impl Solution {
    pub fn new(nums: Vec<i32>) -> Self {
        Self { original: nums }
    }

    pub fn reset(&self) -> Vec<i32> {
        self.original.clone()
    }

    pub fn shuffle(&mut self) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void solutionTest()
    //   Solution solution = new Solution(new int[] {1, 2, 3});
    //   int[] shuffled = solution.shuffle();
    //   Arrays.sort(shuffled);
    //   assertThat(shuffled, equalTo(new int[] {1, 2, 3}));
    //   assertThat(solution.reset(), equalTo(new int[] {1, 2, 3}));
    #[test]
    fn test_solution_test() {
        // TODO: 翻译 Java 测试
    }
}
