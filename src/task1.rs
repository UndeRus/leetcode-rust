/*
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.
 */
use std::collections::HashMap;

struct Solution;

impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            let mut map: HashMap<i32, i32> = HashMap::new();
            for (i, v) in nums.iter().enumerate() {
                let complement = target - *v;
                if map.contains_key(&complement) {
                    return vec![map[&complement], i as i32];
                }
                map.insert(*v, i as i32);
            }
            vec![]
        }
}

#[cfg(test)]
mod tests {
    use crate::task1::Solution;

    #[test]
    fn test() {
        let result = Solution::two_sum(
            vec![2,7,11,15],
            9);
        assert_eq!(result, vec![0, 1]);
    }
}