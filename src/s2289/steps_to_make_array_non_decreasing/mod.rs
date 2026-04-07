// Problem 2289: steps to make array non decreasing

pub struct Solution;

impl Solution {
    pub fn total_steps(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut stack: Vec<(i32, i32)> = Vec::new(); // (value, max_steps_to_remove_this_element)

        for num in nums {
            let mut steps = 0;
            // Pop all elements smaller than or equal to current
            // The current element "defeats" them and takes their max steps + 1
            while !stack.is_empty() && stack.last().unwrap().0 <= num {
                steps = steps.max(stack.last().unwrap().1);
                stack.pop();
            }

            // If stack is not empty, current element will eventually be removed
            // by the top of the stack
            if !stack.is_empty() {
                steps += 1;
                result = result.max(steps);
            } else {
                steps = 0;
            }

            stack.push((num, steps));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void totalSteps()
    //   assertThat(
    //   new Solution().totalSteps(new int[] {5, 3, 4, 4, 7, 3, 6, 11, 8, 5, 11}),
    //   equalTo(3));
    #[test]
    fn test_total_steps() {
        assert_eq!(
            Solution::total_steps(vec
![5, 3, 4, 4, 7, 3, 6, 11, 8, 5, 11]),
            3
        );
    }

    // Java: void totalSteps2()
    //   assertThat(new Solution().totalSteps(new int[] {4, 5, 7, 7, 13}), equalTo(0));
    #[test]
    fn test_total_steps2() {
        assert_eq!(Solution::total_steps(vec
![4, 5, 7, 7, 13]), 0);
    }
}
