impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut total = nums[0];

        let mut current_max = nums[0];
        let mut max_sum = nums[0];

        let mut current_min = nums[0];
        let mut min_sum = nums[0];

        for &num in nums.iter().skip(1) {
            total += num;

            current_max = num.max(current_max + num);
            max_sum = max_sum.max(current_max);

            current_min = num.min(current_min + num);
            min_sum = min_sum.min(current_min);
        }

        if max_sum < 0 {
            max_sum
        } else {
            max_sum.max(total - min_sum)
        }
    }
}