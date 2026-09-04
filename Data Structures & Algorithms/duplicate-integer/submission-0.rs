impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        let n = nums.len();
        for i in 0..n {
            if set.contains(&nums[i]) {
                return true;
            } else {
                set.insert(nums[i]);
            }
        }

        false
    }
}
