// Problem 3028: Ant on the Boundary
// #Easy #Array #Simulation #Prefix_Sum

pub struct Solution;

impl Solution {
    pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut num: i32 = 0;
        for &n in &nums {
            num += n;
            if num == 0 {
                ans += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void returnToBoundaryCount()
    //   assertThat(new Solution().returnToBoundaryCount(new int[] {2, 3, -5}), equalTo(1));
    #[test]
    fn test_return_to_boundary_count() {
        assert_eq!(Solution::return_to_boundary_count(vec![2, 3, -5]), 1);
    }

    // Java: void returnToBoundaryCount2()
    //   assertThat(new Solution().returnToBoundaryCount(new int[] {3, 2, -3, -4}), equalTo(0));
    #[test]
    fn test_return_to_boundary_count2() {
        assert_eq!(Solution::return_to_boundary_count(vec![3, 2, -3, -4]), 0);
    }
}
