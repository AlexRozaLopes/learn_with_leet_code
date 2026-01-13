use std::cmp::max;

pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut z = Vec::new();
    nums.retain(|x| {
        if *x != 0 {
            true
        } else {
            z.push(0);
            false
        }
    });
    let _ = &nums.append(&mut z);
}

pub fn is_subsequence(s: String, t: String) -> bool {
    let st = s.chars().collect::<Vec<char>>();
    let ct = t.chars().collect::<Vec<char>>();

    let mut iter = st.iter();
    let mut option = iter.next();

    for i in 0..ct.len() {
        match option {
            None => {}
            Some(c) => {
                if c.eq(&ct[i]) {
                    option = iter.next()
                }
            }
        }
    }

    option.is_none()
}

pub fn pair_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut r = nums.len() - 1;
    let mut l = 0;

    loop {
        if nums[l] + nums[r] > target {
            r -= 1;
        } else if nums[l] + nums[r] < target {
            l += 1;
        }

        return vec![l as i32, r as i32];
    }
}

pub fn is_palindrome(word: String) -> bool {
    let vec = word.chars().collect::<Vec<char>>();
    let mut l = 0;
    let mut r = vec.len() - 1;

    while l < r {
        if vec[l].is_alphanumeric() == vec[r].is_alphanumeric() {
            if vec[l] != vec[r] {
                return false;
            }
            l += 1;
            r -= 1;
        } else {
            if !vec[l].is_alphanumeric() {
                l += 1;
            }
            if !vec[r].is_alphanumeric() {
                r -= 1;
            }
        }
    }
    return true;
}

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = height.len() - 1;

    let mut max_area = 0;
    let mut area = 0;

    loop {
        area = std::cmp::min(height[l], height[r]) * (r as i32 - l as i32);

        max_area = max(max_area, area);

        if height[l] < height[r] {
            l += 1;
        } else if height[r] < height[l] {
            r -= 1;
        } else {
            l += 1;
            r -= 1;
        }

        if l >= r {
            break;
        }
    }
    max_area
}

pub fn is_happy_number(number: i32) -> bool {
    let (mut slow, mut fast) = (number, number);

    loop {
        slow = get_next_sum(slow);
        fast = get_next_sum(get_next_sum(fast));

        println!("slow = {}, fast = {}", slow, fast);

        if fast == 1 {
            return true;
        }
        if slow == fast {
            return false;
        }
    }
}

fn get_next_sum(number: i32) -> i32 {
    let mut sum = 0;
    number.to_string().chars().for_each(|c| {
        let sq = c.to_string().parse::<i32>().unwrap() * c.to_string().parse::<i32>().unwrap();
        sum += sq;
    });
    sum
}

#[cfg(test)]
mod tests {
    use crate::leet_code75::two_points::code::*;

    #[test]
    fn test_is_subsequence() {
        assert_eq!(
            is_subsequence(String::from("abc"), String::from("ahbgdc")),
            true
        );
    }

    #[test]
    fn test_pair_sum() {
        assert_eq!(pair_sum(vec![1, 2, 3].to_vec(), 4), vec![0, 2]);
    }

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome("arara!".to_string()), true);
    }

    #[test]
    fn test_happy_number() {
        assert_eq!(is_happy_number(23), true);
    }
}
