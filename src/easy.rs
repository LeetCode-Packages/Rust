use super::utils::*;
use std::cell::RefCell;
use std::rc::Rc;

/// # 1. Two Sum
/// Given an array of integers `nums` and an integer `target`, return indices of the two numbers such that they add up to `target`.
///
/// You may assume that each input would have **exactly one solution**, and you may not use the same element twice.
///
/// You can return the answer in any order.
///
/// #### Example 1:
/// > **Input:** ```nums = [2,7,11,15], target = 9```
///
/// > **Output:** ```[0,1]```
///
/// > **Explanation:** ```Because nums[0] + nums[1] == 9, we return [0, 1].```
/// #### Example 2:
/// > **Input:** ```nums = [3,2,4], target = 6```
///
/// > **Output:** ```[1,2]```
/// #### Example 3:
/// > **Input:** ```nums = [3,3], target = 6```
///
/// > **Output:** ```[0,1]```
///
/// #### Constraints:
/// - <code>2 <= nums.length <= 10<sup>4</sup></code>
/// - <code>-10<sup>9</sup> <= nums\[i\] <= 10<sup>9</sup></code>
/// - <code>-10<sup>9</sup> <= target <= 10<sup>9</sup></code>
/// - **Only one valid answer exists.**
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums: Vec<(usize, i32)> = nums.into_iter().enumerate().collect::<Vec<(usize, i32)>>();
    nums.sort_unstable_by_key(|&(_, v)| v);
    for (i, (k, v)) in nums.iter().enumerate() {
        match nums[i + 1..].binary_search_by_key(&(target - *v), |&(_, b)| b) {
            Ok(j) => {
                return vec![*k as i32, nums[j + i + 1].0 as i32];
            }
            Err(_) => {}
        }
    }
    unreachable!();
}

/// 9. Palindrome Number
/// Given an integer x, return true if x is a palindrome, and false otherwise.
pub fn palindrome_number(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let mut tmp = x;
    let mut s: i32 = 0;
    while tmp != 0 {
        s = s * 10 + tmp % 10;
        tmp /= 10;
    }
    x == s
}

pub fn roman_to_integer(s: String) -> i32 {
    s.chars().rfold(0, |acc, c| {
        acc + match c {
            'I' if acc >= 5 => -1,
            'I' => 1,
            'V' => 5,
            'X' if acc >= 50 => -10,
            'X' => 10,
            'L' => 50,
            'C' if acc >= 500 => -100,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    })
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    strs.into_iter()
        .reduce(|acc, cur| {
            acc.chars()
                .zip(cur.chars())
                .take_while(|(a, c)| a == c)
                .map(|(c, _)| c)
                .collect()
        })
        .unwrap()
}

pub fn valid_parentheses(s: String) -> bool {
    let mut stack = Vec::new();
    for i in s.chars() {
        match i {
            '{' => stack.push('}'),
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '}' | ')' | ']' if Some(i) != stack.pop() => return false,
            _ => (),
        }
    }
    return stack.is_empty();
}

pub fn merge_two_sorted_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (Some(n), None) | (None, Some(n)) => Some(n),
        (Some(list1), Some(list2)) => {
            if list1.val >= list2.val {
                Some(Box::new(ListNode {
                    val: list2.val,
                    next: merge_two_sorted_lists(Some(list1), list2.next),
                }))
            } else {
                Some(Box::new(ListNode {
                    val: list1.val,
                    next: merge_two_sorted_lists(list1.next, Some(list2)),
                }))
            }
        }
    }
}

pub fn remove_duplicates_from_sorted_array(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&x| x != val);
    return nums.len() as i32;
}

pub fn find_the_index_of_the_first_occurrence_in_a_string(haystack: String, needle: String) -> i32 {
    haystack.find(&needle).map_or(-1, |index| index as i32)
}

pub fn search_insert_position(nums: Vec<i32>, target: i32) -> i32 {
    nums.partition_point(|&num| num < target) as i32
}

pub fn length_of_last_word(s: String) -> i32 {
    s.trim_end()
        .bytes()
        .rev()
        .take_while(|&b| b != b' ')
        .count() as i32
}

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    for v in digits.iter_mut().rev() {
        let sum = *v + 1;
        *v = sum % 10;
        if sum < 10 {
            return digits;
        }
    }
    [&vec![1], &digits[..]].concat()
}

pub fn add_binary(a: String, b: String) -> String {
    let mut a = &a;
    let mut b = &b;
    if a.len() < b.len() {
        std::mem::swap(&mut a, &mut b);
    }

    let res = a
        .as_bytes()
        .rchunks(127)
        .zip(
            b.as_bytes()
                .rchunks(127)
                .chain(std::iter::repeat(&[b'0'; 127][..])),
        )
        .fold((String::new(), 0), |s, (a, b)| {
            let sum = unsafe {
                s.1 + u128::from_str_radix(std::str::from_utf8_unchecked(a), 2).unwrap_or(0)
                    + u128::from_str_radix(std::str::from_utf8_unchecked(b), 2).unwrap_or(0)
            };
            (
                format!("{:0127b}", sum & 0x7fffffffffffffffffffffffffffffff) + &s.0,
                sum >> 127,
            )
        });

    if res.1 == 1 {
        "1".to_string() + &res.0
    } else {
        let str = res.0.trim_start_matches("0").to_string();
        if str.len() > 0 {
            str
        } else {
            "0".to_string()
        }
    }
}

pub fn sqrt_x(x: i32) -> i32 {
    if x < 2 {
        return x;
    }

    let mut n = x / 2;

    loop {
        let y = (n + x / n) / 2;
        if y >= n {
            return n;
        }
        n = y;
    }
}

pub fn climbing_stairs(n: i32) -> i32 {
    (0..n).fold((1, 0), |(res, prev), _| (res + prev, res)).0
}

pub fn remove_duplicates_from_sorted_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    head.map(|mut ln| {
        let mut cur = ln.as_mut();
        while let Some(next) = cur.next.as_mut() {
            if cur.val == next.val {
                cur.next = next.next.take();
            } else {
                cur = cur.next.as_mut().unwrap();
            }
        }
        ln
    })
}

pub fn merge_sorted_array(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let (mut m, mut n) = (m as usize, n as usize);
    while n > 0 {
        if m > 0 && nums1[m - 1] > nums2[n - 1] {
            nums1[m + n - 1] = nums1[m - 1];
            m -= 1;
        } else {
            nums1[m + n - 1] = nums2[n - 1];
            n -= 1;
        }
    }
}

pub fn binary_tree_inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut curr = root;
    let mut stack = Vec::<Rc<RefCell<TreeNode>>>::new();
    let mut res = vec![];

    while curr.is_some() || !stack.is_empty() {
        while let Some(node_rc) = curr {
            let left = node_rc.borrow_mut().left.take();
            stack.push(node_rc);
            curr = left;
        }

        let node_rc = stack.pop().unwrap();
        let mut node_ref = node_rc.borrow_mut();
        res.push(node_ref.val);
        curr = node_ref.right.take();
    }
    res
}

pub fn same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    p == q
}

pub fn symmetric_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if root.is_none() {
        return true;
    }
    let b = root.as_ref().unwrap().borrow();
    let mut v = vec![(b.left.clone(), b.right.clone())];
    while let Some(tuple) = v.pop() {
        match tuple {
            (None, None) => (),
            (Some(n1), Some(n2)) => {
                let b1 = n1.borrow();
                let b2 = n2.borrow();
                if b1.val != b2.val {
                    return false;
                }
                v.push((b1.left.clone(), b2.right.clone()));
                v.push((b1.right.clone(), b2.left.clone()));
            }
            _ => return false,
        }
    }
    true
}

pub fn maximum_depth_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut s = vec![(root.unwrap(), 1)];
    let mut max_depth = 0;
    while let Some((rc, depth)) = s.pop() {
        max_depth = max_depth.max(depth);

        let mut rc_mut = rc.borrow_mut();
        if let Some(left) = rc_mut.left.take() {
            s.push((left, depth + 1));
        }
        if let Some(right) = rc_mut.right.take() {
            s.push((right, depth + 1));
        }
    }
    max_depth
}

pub fn convert_sorted_array_to_binary_search_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let n = nums.len();

    match n {
        0 => None,
        _ => {
            let m = n / 2;
            let mut node = TreeNode::new(nums[m]);
            node.left = convert_sorted_array_to_binary_search_tree(nums[..m].to_vec());
            node.right = convert_sorted_array_to_binary_search_tree(nums[m + 1..].to_vec());

            Some(Rc::new(RefCell::new(node)))
        }
    }
}

pub fn balanced_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut dstack = Vec::new();
    let mut stack = Vec::new();
    stack.push((1 as i32, 0 as i32, false, false, root));
    while let Some((depth, left_depth, seen_left, seen_right, node)) = stack.pop() {
        if let Some(nval) = node.clone() {
            if !seen_left {
                stack.push((depth, left_depth, true, false, node.clone()));
                stack.push((
                    depth + 1,
                    left_depth,
                    false,
                    false,
                    nval.borrow().left.clone(),
                ));
            } else if !seen_right {
                stack.push((depth, dstack.pop().unwrap(), true, true, node.clone()));
                stack.push((
                    depth + 1,
                    left_depth,
                    false,
                    false,
                    nval.borrow().right.clone(),
                ));
            } else {
                let ldepth = left_depth;
                let rdepth = dstack.pop().unwrap();
                if 1 < (ldepth - rdepth).abs() {
                    return false;
                }
                dstack.push(ldepth.max(rdepth));
            }
        } else {
            dstack.push(depth - 1);
        }
    }
    return true;
}

pub fn minimum_depth_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            let left = minimum_depth_of_binary_tree(node.as_ref().borrow().left.clone());
            let right = minimum_depth_of_binary_tree(node.as_ref().borrow().right.clone());
            if left == 0 || right == 0 {
                return left.max(right) + 1;
            }
            left.min(right) + 1
        }
        None => 0,
    }
}

pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    root.map_or(false, |root| match &*root.borrow() {
        &TreeNode {
            val,
            left: None,
            right: None,
        } => val == target_sum,
        &TreeNode {
            val,
            ref left,
            ref right,
        } => path_sum(left.clone(), target_sum - val) || path_sum(right.clone(), target_sum - val),
    })
}

pub fn pascals_triangle(num_rows: i32) -> Vec<Vec<i32>> {
    todo!();
}

pub fn pascals_triangle_ii(row_index: i32) -> Vec<i32> {
    todo!();
}

pub fn best_time_to_buy_and_sell_stock(prices: Vec<i32>) -> i32 {
    todo!();
}

pub fn valid_palindrome(s: String) -> bool {
    todo!();
}

pub fn single_number(nums: Vec<i32>) -> i32 {
    todo!();
}

// Linked List Cycle - not possible with current ListNode definition

pub fn binary_tree_preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    todo!();
}

pub fn binary_tree_postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    todo!();
}

// Read N Characters Given Read4 - paywalled

// Intersection of Two Linked Lists - not possible with current ListNode definition

// Missing Ranges - paywalled

pub fn excel_sheet_column_title(column_number: i32) -> String {
    todo!();
}

pub fn majority_element(nums: Vec<i32>) -> i32 {
    todo!();
}

// Two Sum III Data Structure Design - paywalled

pub fn excel_sheet_column_number(column_title: String) -> i32 {
    todo!();
}

pub fn reverse_bits(x: u32) -> u32 {
    todo!();
}

pub fn number_of_1_bits(n: i32) -> i32 {
    todo!();
}

pub fn happy_number(n: i32) -> bool {
    todo!();
}

pub fn remove_linked_list_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    todo!();
}

pub fn isomorphic_strings(s: String, t: String) -> bool {
    todo!();
}

pub fn reverse_linked_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    todo!();
}

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    todo!();
}

pub fn contains_duplicate_ii(nums: Vec<i32>, k: i32) -> bool {
    todo!();
}

pub fn count_complete_tree_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    todo!();
}

// Implement Stack using Queues - not applicable

pub fn invert_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    todo!();
}

pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    todo!();
}

pub fn power_of_two(n: i32) -> bool {
    todo!();
}

// Implement Queue using Stacks - not applicable

pub fn palindrome_linked_list(head: Option<Box<ListNode>>) -> bool {
    todo!();
}

pub fn valid_anagram(s: String, t: String) -> bool {
    todo!();
}

// Shortest Word Distance - paywalled

// Strobogrammatic Number - paywalled

// Meeting Rooms - paywalled
