// Problem 3396: minimum number of operations to make elements in array distinct

pub struct Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut dupct = 0;
        for &num in &nums {
            *map.entry(num).or_insert(0) += 1;
            if map[&num] == 2 {
                dupct += 1;
            }
        }
        let n = nums.len();
        let mut i = 0;
        let mut op = 0;
        while dupct > 0 {
            op += 1;
            let limit = (n).min(i + 3);
            while i < limit {
                let val = map[&nums[i]];
                if val == 2 {
                    dupct -= 1;
                }
                map.insert(nums[i], val - 1);
                i += 1;
            }
        }
        op
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minimumOperations()
    //   assertThat(
    //   new Solution().minimumOperations(new int[] {1, 2, 3, 4, 2, 3, 3, 5, 7}),
    //   equalTo(2));
    #[test]
    fn test_minimum_operations() {
        // TODO: 翻译 Java 测试
    }

    // Java: void minimumOperations2()
    //   assertThat(new Solution().minimumOperations(new int[] {4, 5, 6, 4, 4}), equalTo(2));
    #[test]
    fn test_minimum_operations2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void minimumOperations3()
    //   assertThat(new Solution().minimumOperations(new int[] {6, 7, 8, 9}), equalTo(0));
    #[test]
    fn test_minimum_operations3() {
        // TODO: 翻译 Java 测试
    }
}
