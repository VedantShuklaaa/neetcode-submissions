impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut sum = 0;

        for num in nums {
            sum += num;
            max = sum.max(max);

            if sum < 0 {
                sum = 0;
            }
        }

        max
    }
}