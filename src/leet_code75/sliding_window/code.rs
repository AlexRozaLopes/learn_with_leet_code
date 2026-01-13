pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let k = k as usize;
    let mut window_sum: i32 = nums[..k].iter().sum();
    let mut max_sum = window_sum;

    for i in k..nums.len() {
        window_sum += nums[i];
        window_sum -= nums[i - k];
        max_sum = max_sum.max(window_sum);
    }

    max_sum as f64 / k as f64
}


#[cfg(test)]
mod tests {
    use crate::leet_code75::sliding_window::code::find_max_average;

    #[test]
    fn test_find_max_average() {
        assert_eq!(find_max_average(vec![1,12,-5,-6,50,3], 4), 12.75000);
    }
}
