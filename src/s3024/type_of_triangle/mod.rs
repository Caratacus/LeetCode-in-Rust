// Problem 3024: Type of Triangle
// #Easy #Array #Math #Sorting

pub struct Solution;

impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        if nums[0] + nums[1] > nums[2]
            && nums[1] + nums[2] > nums[0]
            && nums[2] + nums[0] > nums[1]
        {
            if nums[0] == nums[1] && nums[1] == nums[2] {
                "equilateral".to_string()
            } else if nums[0] == nums[1] || nums[0] == nums[2] || nums[2] == nums[1] {
                "isosceles".to_string()
            } else {
                "scalene".to_string()
            }
        } else {
            "none".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void triangleType()
    //   assertThat(new Solution().triangleType(new int[] {3, 3, 3}), equalTo("equilateral"));
    #[test]
    fn test_triangle_type() {
        assert_eq!(Solution::triangle_type(vec![3, 3, 3]), "equilateral");
    }

    // Java: void triangleType2()
    //   assertThat(new Solution().triangleType(new int[] {3, 4, 5}), equalTo("scalene"));
    #[test]
    fn test_triangle_type2() {
        assert_eq!(Solution::triangle_type(vec![3, 4, 5]), "scalene");
    }

    // Java: void triangleType3()
    //   assertThat(new Solution().triangleType(new int[] {5, 5, 3}), equalTo("isosceles"));
    #[test]
    fn test_triangle_type3() {
        assert_eq!(Solution::triangle_type(vec![5, 5, 3]), "isosceles");
    }

    // Java: void triangleType4()
    //   assertThat(new Solution().triangleType(new int[] {1, 2, 3}), equalTo("none"));
    #[test]
    fn test_triangle_type4() {
        assert_eq!(Solution::triangle_type(vec![1, 2, 3]), "none");
    }

    // Java: void triangleType5()
    //   assertThat(new Solution().triangleType(new int[] {100, 100, 100}), equalTo("equilateral"));
    #[test]
    fn test_triangle_type5() {
        assert_eq!(Solution::triangle_type(vec![100, 100, 100]), "equilateral");
    }

    // Java: void triangleType6()
    //   assertThat(new Solution().triangleType(new int[] {7, 10, 7}), equalTo("isosceles"));
    #[test]
    fn test_triangle_type6() {
        assert_eq!(Solution::triangle_type(vec![7, 10, 7]), "isosceles");
    }

    // Java: void triangleType7()
    //   assertThat(new Solution().triangleType(new int[] {0, 4, 4}), equalTo("none"));
    #[test]
    fn test_triangle_type7() {
        assert_eq!(Solution::triangle_type(vec![0, 4, 4]), "none");
    }

    // Java: void triangleType8()
    //   assertThat(new Solution().triangleType(new int[] {-3, 4, 5}), equalTo("none"));
    #[test]
    fn test_triangle_type8() {
        assert_eq!(Solution::triangle_type(vec![-3, 4, 5]), "none");
    }

    // Java: void triangleType9()
    //   // 2 + 3 = 5 -> equals, not greater -> invalid
    //   assertThat(new Solution().triangleType(new int[] {2, 3, 5}), equalTo("none"));
    #[test]
    fn test_triangle_type9() {
        assert_eq!(Solution::triangle_type(vec![2, 3, 5]), "none");
    }

    // Java: void triangleType10()
    //   assertThat(new Solution().triangleType(new int[] {5, 3, 4}), equalTo("scalene"));
    #[test]
    fn test_triangle_type10() {
        assert_eq!(Solution::triangle_type(vec![5, 3, 4]), "scalene");
    }

    // Java: void triangleType11()
    //   assertThat(new Solution().triangleType(new int[] {4, 6, 6}), equalTo("isosceles"));
    #[test]
    fn test_triangle_type11() {
        assert_eq!(Solution::triangle_type(vec![4, 6, 6]), "isosceles");
    }

    // Java: void triangleType12()
    //   assertThat(new Solution().triangleType(new int[] {0, 0, 0}), equalTo("none"));
    #[test]
    fn test_triangle_type12() {
        assert_eq!(Solution::triangle_type(vec![0, 0, 0]), "none");
    }

    // Java: void triangleType13()
    //   assertThat(new Solution().triangleType(new int[] {5, 5, 6}), equalTo("isosceles"));
    #[test]
    fn test_triangle_type13() {
        assert_eq!(Solution::triangle_type(vec![5, 5, 6]), "isosceles");
    }

    // Java: void triangleType14()
    //   assertThat(new Solution().triangleType(new int[] {10, 11, 12}), equalTo("scalene"));
    #[test]
    fn test_triangle_type14() {
        assert_eq!(Solution::triangle_type(vec![10, 11, 12]), "scalene");
    }

    // Java: void triangleType15()
    //   assertThat(new Solution().triangleType(new int[] {1, 10, 12}), equalTo("none"));
    #[test]
    fn test_triangle_type15() {
        assert_eq!(Solution::triangle_type(vec![1, 10, 12]), "none");
    }
}
