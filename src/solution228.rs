struct Solution;

// https://leetcode.com/problems/h-index/description/?envType=study-plan-v2&id=top-interview-150
// 3,0,6,1,5
// 0,1,3,5,6

// 0,1
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result = vec![];
        if nums.len() == 0 {
            return result
        }
        let mut s = nums[0];
        for (i, _) in nums.iter().enumerate() {
            if i == nums.len() - 1 || nums[i + 1] as i64 - nums[i] as i64 > 1 {
                if s == nums[i] {
                    result.push(format!("{}", s))
                } else {
                    result.push(format!("{}->{}", s, nums[i]))
                }
                if i < nums.len() - 1 {
                    s = nums[i + 1]
                }
            }
        }
        return result
    }
}

#[cfg(test)]
mod tests {
    use crate::solution228::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]), vec!["0->2", "4->5", "7"]);
        assert_eq!(Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]), vec!["0", "2->4", "6", "8->9"]);
    }
}