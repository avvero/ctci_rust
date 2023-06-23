struct Solution;

// https://leetcode.com/problems/h-index/description/?envType=study-plan-v2&id=top-interview-150
// 3,0,6,1,5
// 0,1,3,5,6

// 0,1
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        return vec![]
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