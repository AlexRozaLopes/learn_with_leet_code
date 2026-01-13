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

pub fn max_vowels(s: String, k: i32) -> i32 {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let chars = s.chars().collect::<Vec<char>>();
    let mut max_vowels = chars[..k as usize].iter().filter(|x| vowels.contains(x)).count() as i32;

    let mut current_vowels = max_vowels;

    for i in k as usize..chars.len() {
        println!("max_vowels: {max_vowels}");
        println!("current_vowels: {current_vowels}");
        if vowels.contains(&chars[i]) {
            current_vowels +=1;
        }

        if vowels.contains(&chars[i - k as usize]) {
            current_vowels -=1;
        }
        max_vowels = max_vowels.max(current_vowels);
    }
    max_vowels
}

pub fn max_vowels_function(s: String, k: i32) -> i32 {
    let vowels: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];
    let k: usize = k as usize;
    let s_bytes: &[u8] = s.as_bytes();
    let mut current_vowels: usize = s_bytes
        .iter()
        .take(k)
        .filter(|&byte| vowels.contains(byte))
        .count();

    let mut n_max_vowels: usize = current_vowels;

    s_bytes
        .iter()
        .skip(k)
        .enumerate()
        .for_each(|(i, e): (usize, &u8)| {
            if vowels.contains(e) {
                current_vowels += 1;
            }
            if vowels.contains(&&s_bytes[i]) {
                current_vowels -= 1;
            }
            n_max_vowels = n_max_vowels.max(current_vowels);
        });

    n_max_vowels as i32
}

#[cfg(test)]
mod tests {
    use crate::leet_code75::sliding_window::code::{find_max_average, max_vowels};

    #[test]
    fn test_find_max_average() {
        assert_eq!(find_max_average(vec![1,12,-5,-6,50,3], 4), 12.75000);
    }

    #[test]
    fn test_max_vowels() {
        assert_eq!(max_vowels("abciiidef".to_string(), 3), 3);
        assert_eq!(max_vowels("leetcode".to_string(), 2), 2);
    }
}
