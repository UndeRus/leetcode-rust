impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let sum1: i32 = nums1.iter().sum();
        let sum2: i32  =nums2.iter().sum();

        dbg!(sum1, sum2);

        let len1 = nums1.len() as f64;
        let len2 = nums2.len() as f64;
        ((sum1 + sum2) as f64 / (len1 + len2)) as f64
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1,3], vec![2]), 2_f64);

        assert_eq!(Solution::find_median_sorted_arrays(vec![1,3], vec![2,7]), 3.25_f64);
    }
}