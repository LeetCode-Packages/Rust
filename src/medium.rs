use super::utils::*;

/// https://leetcode.com/problems/add-two-numbers/
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => None,
        (Some(n), None) | (None, Some(n)) => Some(n),
        (Some(n1), Some(n2)) => {
            let sum = n1.val + n2.val;
            if sum < 10 {
                Some(Box::new(ListNode {
                    val: sum,
                    next: add_two_numbers(n1.next, n2.next),
                }))
            } else {
                let carry = Some(Box::new(ListNode::new(1)));
                Some(Box::new(ListNode {
                    val: sum - 10,
                    next: add_two_numbers(add_two_numbers(carry, n1.next), n2.next),
                }))
            }
        }
    }
}

/// https://leetcode.com/problems/longest-substring-without-repeating-characters/
pub fn longest_substring_without_repeating_characters(s: String) -> i32 {
    let mut max_len: usize = 0;
    let mut pos: [usize; 128] = [0; 128];
    let mut start: usize = 0;

    for (end, ch) in s.chars().enumerate() {
        start = start.max(pos[ch as usize]);
        max_len = max_len.max(end - start + 1);
        pos[ch as usize] = end + 1;
    }
    return max_len as i32;
}

/// https://leetcode.com/problems/longest-palindromic-substring/
pub fn longest_palindromic_substring(s: String) -> String {
    fn is_palidrome(s: &[u8]) -> bool {
        s.iter().zip(s.iter().rev()).all(|(l, r)| l == r)
    }

    for size in (1..=s.len()).rev() {
        match s
            .as_bytes()
            .windows(size)
            .find(|substr| is_palidrome(substr))
        {
            Some(pal) => return String::from_utf8(pal.to_vec()).unwrap(),
            None => continue,
        }
    }
    "".to_string()
}

/// https://leetcode.com/problems/zigzag-conversion/
pub fn zigzag_conversion(s: String, num_rows: i32) -> String {
    let mut zigzags: Vec<_> = (0..num_rows)
        .chain((1..num_rows - 1).rev())
        .cycle()
        .zip(s.chars())
        .collect();
    zigzags.sort_by_key(|&(row, _)| row);
    zigzags.into_iter().map(|(_, c)| c).collect()
}

/// https://leetcode.com/problems/reverse-integer/
pub fn reverse_integer(x: i32) -> i32 {
    let mut res: i32 = 0;
    let mut cur: i32 = x;

    while cur != 0 {
        match res.checked_mul(10) {
            None => return 0,
            Some(tmp) => match tmp.checked_add(cur % 10) {
                None => return 0,
                Some(fine) => {
                    res = fine;
                }
            },
        }
        cur = cur / 10;
    }
    res
}

/// https://leetcode.com/problems/string-to-integer-atoi/
pub fn string_to_integer_atoi(s: String) -> i32 {
    let s = s.trim_start();
    let (s, sign) = match s.strip_prefix('-') {
        Some(s) => (s, -1),
        None => (s.strip_prefix('+').unwrap_or(s), 1),
    };
    s.chars()
        .map(|c| c.to_digit(10))
        .take_while(Option::is_some)
        .flatten()
        .fold(0, |acc, digit| {
            acc.saturating_mul(10).saturating_add(sign * digit as i32)
        })
}

/// https://leetcode.com/problems/container-with-most-water/
pub fn container_with_most_water(height: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut iter = height.iter().enumerate();
    let mut p1 = iter.next();
    let mut p2 = iter.next_back();
    while let (Some((i, h1)), Some((j, h2))) = (p1, p2) {
        result = result.max(h1.min(h2) * (j - i) as i32);
        if h1 < h2 {
            p1 = iter.next();
        } else {
            p2 = iter.next_back();
        }
    }
    result
}

/// https://leetcode.com/problems/integer-to-roman/
pub fn integer_to_roman(num: i32) -> String {
    const ONES: [&str; 10] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
    const TENS: [&str; 10] = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
    const CENT: [&str; 10] = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
    const MILS: [&str; 4] = ["", "M", "MM", "MMM"];

    format!(
        "{}{}{}{}",
        MILS[(num / 1000 % 10) as usize],
        CENT[(num / 100 % 10) as usize],
        TENS[(num / 10 % 10) as usize],
        ONES[(num % 10) as usize]
    )
}

/// https://leetcode.com/problems/3sum/
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let max = nums.len();
    let mut wnums = nums.clone();
    wnums.sort();
    let mut result: Vec<Vec<i32>> = Vec::new();
    for i in 0..max {
        if i > 0 && wnums[i] == wnums[i - 1] {
            continue;
        }
        let mut j = i + 1;
        let mut k = max - 1;
        while j < k {
            let sum = wnums[i] + wnums[j] + wnums[k];
            if sum == 0 {
                result.push(vec![wnums[i], wnums[j], wnums[k]]);
                j += 1;
                k -= 1;
                while j < k && wnums[j] == wnums[j - 1] {
                    j += 1
                }
                while j < k && wnums[k] == wnums[k + 1] {
                    k -= 1
                }
            } else {
                if sum < 0 {
                    j += 1
                } else {
                    k -= 1
                }
            }
        }
    }
    result
}

/// https://leetcode.com/problems/3sum-closest/
pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut arr: Vec<i32> = nums.clone();
    arr.sort();
    let n: usize = arr.len();
    let mut min_diff: i32 = i32::MAX;
    let mut min_sum: i32 = i32::MAX;
    for i in 0..n - 2 {
        let mut j = i + 1;
        let mut k = n - 1;
        while k > j {
            let _sum = arr[i] + arr[j] + arr[k];
            let _diff = _sum - target;
            if _diff == 0 {
                return _sum;
            } else if _diff < 0 {
                if min_diff > { -_diff } {
                    min_diff = -_diff;
                    min_sum = _sum;
                }
                j += 1;
            } else {
                if min_diff > _diff {
                    min_diff = _diff;
                    min_sum = _sum;
                }
                k -= 1;
            }
        }
    }
    min_sum
}
