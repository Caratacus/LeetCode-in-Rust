// Problem 0703: kth largest element in a stream

pub struct KthLargest {
    k: i32,
    nums: Vec<i32>,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        todo!()
    }

    pub fn add(&mut self, val: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void kthLargestTest()
    //   KthLargest kthLargest = new KthLargest(3, new int[] {4, 5, 8, 2});
    //   assertThat(kthLargest.add(3), equalTo(4));
    //   assertThat(kthLargest.add(5), equalTo(5));
    //   assertThat(kthLargest.add(10), equalTo(5));
    //   assertThat(kthLargest.add(9), equalTo(8));
    //   ... (1 more lines)
    #[test]
    fn test_kth_largest_test() {
        // TODO: 翻译 Java 测试
    }
}
