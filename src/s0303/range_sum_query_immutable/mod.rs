// Problem 0303: range sum query immutable

pub struct NumArray {
    nums: Vec<i32>,
}

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        todo!()
    }

    pub fn sum_range(&mut self, i: i32, j: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void numArray()
    //   NumArray numArray = new NumArray(new int[] {-2, 0, 3, -5, 2, -1});
    //   // return (-2) + 0 + 3 = 1
    //   assertThat(numArray.sumRange(0, 2), equalTo(1));
    //   // return 3 + (-5) + 2 + (-1) = -1
    //   assertThat(numArray.sumRange(2, 5), equalTo(-1));
    //   ... (2 more lines)
    #[test]
    fn test_num_array() {
        // TODO: 翻译 Java 测试
    }
}
