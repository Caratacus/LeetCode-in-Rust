// Problem 0307: range sum query mutable

pub struct NumArray {
    nums: Vec<i32>,
}

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        todo!()
    }

    pub fn update(&mut self, index: i32, val: i32) -> () {
        todo!()
    }

    pub fn sum_range(&mut self, left: i32, right: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void numArray()
    //   NumArray numArray = new NumArray(new int[] {1, 3, 5});
    //   assertThat(numArray.sumRange(0, 2), equalTo(9));
    //   numArray.update(1, 2);
    //   assertThat(numArray.sumRange(0, 2), equalTo(8));
    #[test]
    fn test_num_array() {
        // TODO: 翻译 Java 测试
    }
}
