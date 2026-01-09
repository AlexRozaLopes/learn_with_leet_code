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

#[cfg(test)]
mod tests {
    use crate::leet_code75::two_points::code::{is_subsequence, pair_sum};

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
}
