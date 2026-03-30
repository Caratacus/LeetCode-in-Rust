// Problem 0295: find median from data stream

pub struct MedianFinder {}

impl MedianFinder {
    pub fn new() -> Self {
        todo!()
    }

    pub fn add_num(&mut self, num: i32) -> () {
        todo!()
    }

    pub fn balance(
        &mut self,
        max_heap: &mut std::collections::BinaryHeap<i32>,
        min_heap: &mut std::collections::BinaryHeap<i32>,
    ) {
        todo!()
    }

    pub fn find_median(&self) -> f64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void medianFinder()
    //   MedianFinder medianFinder = new MedianFinder();
    //   // arr = [1]
    //   medianFinder.addNum(1);
    //   // arr = [1, 2]
    //   medianFinder.addNum(2);
    //   ... (6 more lines)
    #[test]
    fn test_median_finder() {
        // TODO: 翻译 Java 测试
    }

    // Java: void medianFinder2()
    //   MedianFinder medianFinder = new MedianFinder();
    //   medianFinder.addNum(1);
    //   medianFinder.addNum(3);
    //   medianFinder.addNum(-1);
    //   assertThat(medianFinder.findMedian(), equalTo(1.0));
    #[test]
    fn test_median_finder2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void medianFinder3()
    //   MedianFinder medianFinder = new MedianFinder();
    //   medianFinder.addNum(42);
    //   assertThat(medianFinder.findMedian(), equalTo(42.0));
    #[test]
    fn test_median_finder3() {
        // TODO: 翻译 Java 测试
    }

    // Java: void medianFinder4()
    //   MedianFinder medianFinder = new MedianFinder();
    //   medianFinder.addNum(5);
    //   medianFinder.addNum(5);
    //   medianFinder.addNum(5);
    //   medianFinder.addNum(5);
    //   ... (1 more lines)
    #[test]
    fn test_median_finder4() {
        // TODO: 翻译 Java 测试
    }

    // Java: void medianFinder5()
    //   MedianFinder medianFinder = new MedianFinder();
    //   medianFinder.addNum(-5);
    //   medianFinder.addNum(-10);
    //   medianFinder.addNum(-3);
    //   assertThat(medianFinder.findMedian(), equalTo(-5.0));
    #[test]
    fn test_median_finder5() {
        // TODO: 翻译 Java 测试
    }

    // Java: void medianFinder6()
    //   MedianFinder medianFinder = new MedianFinder();
    //   medianFinder.addNum(1000);
    //   medianFinder.addNum(1);
    //   medianFinder.addNum(500);
    //   medianFinder.addNum(0);
    //   ... (1 more lines)
    #[test]
    fn test_median_finder6() {
        // TODO: 翻译 Java 测试
    }

    // Java: void medianFinder7()
    //   MedianFinder medianFinder = new MedianFinder();
    //   medianFinder.addNum(1);
    //   medianFinder.addNum(2);
    //   medianFinder.addNum(3);
    //   medianFinder.addNum(4);
    //   ... (1 more lines)
    #[test]
    fn test_median_finder7() {
        // TODO: 翻译 Java 测试
    }
}
