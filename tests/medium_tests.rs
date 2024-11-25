use rsleetcode::medium::*;
use rsleetcode::utils::*;

#[test]
fn test_add_two_numbers() {
    assert_eq!(
        add_two_numbers(
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 3, next: None }))
                }))
            })),
            Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: Some(Box::new(ListNode { val: 4, next: None }))
                }))
            }))
        ),
        Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 8, next: None }))
            }))
        }))
    );
    assert_eq!(
        add_two_numbers(
            Some(Box::new(ListNode { val: 0, next: None })),
            Some(Box::new(ListNode { val: 0, next: None }))
        ),
        Some(Box::new(ListNode { val: 0, next: None }))
    );
    assert_eq!(
        add_two_numbers(
            Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode {
                                val: 9,
                                next: Some(Box::new(ListNode {
                                    val: 9,
                                    next: Some(Box::new(ListNode { val: 9, next: None }))
                                }))
                            }))
                        }))
                    }))
                }))
            })),
            Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode { val: 9, next: None }))
                    }))
                }))
            }))
        ),
        Some(Box::new(ListNode {
            val: 8,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 0,
                            next: Some(Box::new(ListNode {
                                val: 0,
                                next: Some(Box::new(ListNode {
                                    val: 0,
                                    next: Some(Box::new(ListNode { val: 1, next: None }))
                                }))
                            }))
                        }))
                    }))
                }))
            }))
        }))
    );
}

#[test]
fn test_longest_substring_without_repeating_characters() {
    assert_eq!(
        longest_substring_without_repeating_characters("abcabcbb".to_string()),
        3
    );
    assert_eq!(
        longest_substring_without_repeating_characters("bbbbb".to_string()),
        1
    );
    assert_eq!(
        longest_substring_without_repeating_characters("pwwkew".to_string()),
        3
    );
}

#[test]
fn test_longest_palindromic_substring() {
    assert_eq!(
        longest_palindromic_substring("babad".to_string()),
        "bab".to_string()
    );
    assert_eq!(
        longest_palindromic_substring("cbbd".to_string()),
        "bb".to_string()
    );
}

#[test]
fn test_zigzag_conversion() {
    assert_eq!(
        zigzag_conversion("PAYPALISHIRING".to_string(), 3),
        "PAHNAPLSIIGYIR".to_string()
    );
    assert_eq!(
        zigzag_conversion("PAYPALISHIRING".to_string(), 4),
        "PINALSIGYAHRPI".to_string()
    );
    assert_eq!(zigzag_conversion("A".to_string(), 1), "A".to_string());
}

#[test]
fn test_reverse_integer() {
    assert_eq!(reverse_integer(123), 321);
    assert_eq!(reverse_integer(-123), -321);
    assert_eq!(reverse_integer(120), 21);
}

#[test]
fn test_string_to_integer_atoi() {
    assert_eq!(string_to_integer_atoi("42".to_string()), 42);
    assert_eq!(string_to_integer_atoi(" -042".to_string()), -42);
    assert_eq!(string_to_integer_atoi("1337c0d3".to_string()), 1337);
    assert_eq!(string_to_integer_atoi("0-1".to_string()), 0);
    assert_eq!(string_to_integer_atoi("words and 987".to_string()), 0);
}

#[test]
fn test_container_with_most_water() {
    assert_eq!(
        container_with_most_water(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]),
        49
    );
    assert_eq!(container_with_most_water(vec![1, 1]), 1);
}

#[test]
fn test_integer_to_roman() {
    assert_eq!(integer_to_roman(3749), "MMMDCCXLIX".to_string());
    assert_eq!(integer_to_roman(58), "LVIII".to_string());
    assert_eq!(integer_to_roman(1994), "MCMXCIV".to_string());
}

#[test]
fn test_three_sum() {
    assert_eq!(
        three_sum(vec![-1, 0, 1, 2, -1, -4]),
        vec![vec![-1, -1, 2], vec![-1, 0, 1]]
    );
    assert_eq!(three_sum(vec![0, 1, 1]), vec![vec![0; 0]; 0]);
    assert_eq!(three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
}

#[test]
fn test_three_sum_closest() {
    assert_eq!(three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    assert_eq!(three_sum_closest(vec![0, 0, 0], 1), 0);
}
