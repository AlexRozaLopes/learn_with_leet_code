pub fn reverse_vowels(s: String) -> String {
    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    let mut find_vowels = s
        .chars()
        .filter(|c| vowels.contains(c))
        .collect::<Vec<char>>();

    s.chars()
        .map(|c| {
            if vowels.contains(&c) {
                find_vowels.pop().unwrap()
            } else {
                c
            }
        })
        .collect()
}

pub fn reverse_vowels_best(mut s: String) -> String {
    // From the description, `s` is guaranteed to be ASCII, so this is fine
    let bytes = unsafe { s.as_bytes_mut() };

    let mut iter = bytes.iter_mut();
    while let (Some(left), Some(right)) = (iter.find(|c| is_vowel(c)), iter.rfind(|c| is_vowel(c)))
    {
        std::mem::swap(left, right);
    }

    s
}

fn is_vowel(c: &u8) -> bool {
    matches!(c.to_ascii_lowercase(), b'a' | b'e' | b'i' | b'o' | b'u')
}

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut ret = vec![1; nums.len()];
    let mut l = 0;
    let mut r = nums.len() - 1;

    let mut lv = 1;
    let mut rv = 1;

    loop {
        ret[l] = ret[l] * lv;
        ret[r] = ret[r] * rv;

        println!("{:?}", ret);

        rv = rv * nums[r];
        lv = lv * nums[l];

        println!("rv: {}, lv: {}", rv, lv);

        if r == 0 {
            break;
        }
        l += 1;
        r -= 1;
    }

    ret
}

pub fn increasing_triplet(nums: Vec<i32>) -> bool
{
    let mut first  = i32::MAX;
    let mut second = i32::MAX;

    for &n in nums.iter()
    {
        if n <= first  { first = n; }
        else if n <= second { second = n; }
        else { return true; }
    }

    return false;
}

pub fn reverse_words(s: String) -> String {
    s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
}

#[cfg(test)]
mod tests {
    use crate::leet_code75::array::code::{
        product_except_self, reverse_vowels, reverse_vowels_best, reverse_words,
    };

    #[test]
    fn test_reverse_vowels() {
        assert_eq!(reverse_vowels("IceCreAm".to_string()), "AceCreIm");
    }
    #[test]
    fn test_reverse_vowels_best() {
        assert_eq!(reverse_vowels_best("IceCreAm".to_string()), "AceCreIm");
    }

    #[test]
    fn test_reverse_words() {
        assert_eq!(
            reverse_words("the sky is blue".to_string()),
            "blue is sky the"
        );
    }

    #[test]
    fn test_product_except_self() {
        assert_eq!(product_except_self(Vec::from([1, 2, 3, 4])), [24, 12, 8, 6]);
    }
}
