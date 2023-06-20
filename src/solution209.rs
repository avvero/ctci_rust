struct Solution;

// https://leetcode.com/problems/minimum-size-subarray-sum/description/?envType=study-plan-v2&envId=top-interview-150

impl Solution {

    // 2,3,1,2,4,3

    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut s = 0;
        let mut sum = 0;
        let mut min = std::i32::MAX;
        let mut i:i32 = 0;
        for v in &nums {
            if *v >= target {
                return 1
            }
            sum += v;
            while sum - nums[s] >= target {
                sum -= nums[s];
                s += 1;
            }
            if sum >= target {
                min = min.min(i - s as i32 + 1)
            }
            i += 1;
        }
        if min != std::i32::MAX {
            return min
        } else {
            return 0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solution209::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2,3,1,2,4,3]), 2);
        assert_eq!(Solution::min_sub_array_len(4, vec![1,4,4]), 1);
        assert_eq!(Solution::min_sub_array_len(11, vec![1,1,1,1,1,1,1,1]), 0);
    }
}