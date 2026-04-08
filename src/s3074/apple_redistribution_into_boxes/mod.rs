// Problem 3074: apple redistribution into boxes
// #Easy #Array #Sorting #Greedy #2024_04_15_Time_1_ms_(99.81%)_Space_41.9_MB_(89.46%)

pub struct Solution;

impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32 {
        let mut count = vec![0i32; 51];
        let mut apple_sum: i32 = apple.iter().sum();
        let mut req_capacity = 0;
        let mut max = 0;

        for &c in &capacity {
            count[c as usize] += 1;
            max = std::cmp::max(max, c);
        }

        for i in (0..=max as usize).rev() {
            if count[i] >= 1 {
                while count[i] != 0 {
                    apple_sum -= i as i32;
                    req_capacity += 1;
                    if apple_sum <= 0 {
                        return req_capacity;
                    }
                    count[i] -= 1;
                }
            }
        }
        req_capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minimumBoxes()
    //   assertThat(
    //   new Solution().minimumBoxes(new int[] {1, 3, 2}, new int[] {4, 3, 1, 5, 2}),
    //   equalTo(2));
    #[test]
    fn test_minimum_boxes() {
        assert_eq!(
            Solution::minimum_boxes(vec![1, 3, 2], vec![4, 3, 1, 5, 2]),
            2
        );
    }

    // Java: void minimumBoxes2()
    //   assertThat(
    //   new Solution().minimumBoxes(new int[] {5, 5, 5}, new int[] {2, 4, 2, 7}),
    //   equalTo(4));
    #[test]
    fn test_minimum_boxes2() {
        assert_eq!(
            Solution::minimum_boxes(vec![5, 5, 5], vec![2, 4, 2, 7]),
            4
        );
    }
}
