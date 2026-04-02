// Problem 0398: random pick index

use std::collections::HashMap;

pub struct Solution {
    index_map: HashMap<i32, Vec<usize>>,
}

impl Solution {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut index_map = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            index_map.entry(num).or_insert_with(Vec::new).push(i);
        }
        Self { index_map }
    }

    pub fn pick(&self, target: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void pick()
    //   int[] input = new int[] {1, 2, 3, 3, 3};
    //   Solution solution = new Solution(input);
    //   assertThat(Arrays.asList(2, 3, 4).contains(solution.pick(3)), equalTo(true));
    //   assertThat(solution.pick(1), equalTo(0));
    //   assertThat(Arrays.asList(2, 3, 4).contains(solution.pick(3)), equalTo(true));
    #[test]
    fn test_pick() {
        // TODO: 翻译 Java 测试
    }
}
