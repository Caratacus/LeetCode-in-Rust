// Problem 3159: find occurrences of an element in an array
// #Medium #Array #Hash_Table #2024_05_30_Time_4_ms_(96.74%)_Space_64_MB_(69.94%)

pub struct Solution;

impl Solution {
    pub fn occurrences_of_element(nums: Vec<i32>, queries: Vec<i32>, x: i32) -> Vec<i32> {
        let mut a: Vec<i32> = Vec::new();
        for i in 0..nums.len() {
            if nums[i] == x {
                a.push(i as i32);
            }
        }
        let l = queries.len();
        let mut r = vec![0i32; l];
        for i in 0..l {
            r[i] = if queries[i] as usize > a.len() {
                -1
            } else {
                a[queries[i] as usize - 1]
            };
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void occurrencesOfElement()
    //   assertThat(
    //   new Solution()
    //   .occurrencesOfElement(new int[] {1, 3, 1, 7}, new int[] {1, 3, 2, 4}, 1),
    //   equalTo(new int[] {0, -1, 2, -1}));
    #[test]
    fn test_occurrences_of_element() {
        assert_eq!(
            Solution::occurrences_of_element(vec![1, 3, 1, 7], vec![1, 3, 2, 4], 1),
            vec![0, -1, 2, -1]
        );
    }

    // Java: void occurrencesOfElement2()
    //   assertThat(
    //   new Solution().occurrencesOfElement(new int[] {1, 2, 3}, new int[] {10}, 5),
    //   equalTo(new int[] {-1}));
    #[test]
    fn test_occurrences_of_element2() {
        assert_eq!(
            Solution::occurrences_of_element(vec![1, 2, 3], vec![10], 5),
            vec![-1]
        );
    }
}
