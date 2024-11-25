use std::{cell::RefCell, rc::Rc};

use rsleetcode::easy::*;
use rsleetcode::utils::*;

#[test]
fn test_two_sum() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
}

#[test]
fn test_palindrome_number() {
    assert!(palindrome_number(121));
    assert!(!palindrome_number(-121));
    assert!(!palindrome_number(10));
}

#[test]
fn test_roman_to_integer() {
    assert_eq!(roman_to_integer("III".to_string()), 3);
    assert_eq!(roman_to_integer("LVIII".to_string()), 58);
    assert_eq!(roman_to_integer("MCMXCIV".to_string()), 1994);
}

#[test]
fn test_longest_common_prefix() {
    assert_eq!(
        longest_common_prefix(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string()
        ]),
        "fl".to_string()
    );
    assert_eq!(
        longest_common_prefix(vec![
            "dog".to_string(),
            "racecar".to_string(),
            "car".to_string()
        ]),
        "".to_string()
    )
}

#[test]
fn test_valid_parentheses() {
    assert!(valid_parentheses("()".to_string()));
    assert!(valid_parentheses("()[]{}".to_string()));
    assert!(!valid_parentheses("(]".to_string()));
    assert!(valid_parentheses("([])".to_string()));
}

#[test]
fn test_merge_two_sorted_lists() {
    let list1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));

    let list2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));

    let sol = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 4, next: None })),
                    })),
                })),
            })),
        })),
    }));

    assert_eq!(merge_two_sorted_lists(list1, list2), sol);
    assert_eq!(merge_two_sorted_lists(None, None), None);
    assert_eq!(
        merge_two_sorted_lists(None, Some(Box::new(ListNode { val: 0, next: None }))),
        Some(Box::new(ListNode { val: 0, next: None }))
    );
}

#[test]
fn test_remove_duplicates_from_sorted_array() {
    let sorted_array = &mut vec![1, 1, 2];
    assert_eq!(remove_duplicates_from_sorted_array(sorted_array), 2);
    assert_eq!(sorted_array[0], 1);
    assert_eq!(sorted_array[1], 2);

    let sorted_array = &mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(remove_duplicates_from_sorted_array(sorted_array), 5);
    assert_eq!(sorted_array[0], 0);
    assert_eq!(sorted_array[1], 1);
    assert_eq!(sorted_array[2], 2);
    assert_eq!(sorted_array[3], 3);
    assert_eq!(sorted_array[4], 4);
}

#[test]
fn test_remove_element() {
    let nums = &mut vec![3, 2, 2, 3];
    assert_eq!(remove_element(nums, 3), 2);
    assert_eq!(nums[0], 2);
    assert_eq!(nums[1], 2);

    let nums = &mut vec![0, 1, 2, 2, 3, 0, 4, 2];
    assert_eq!(remove_element(nums, 2), 5);
    assert_eq!(nums[..5].sort(), vec![0, 1, 3, 0, 4].sort());
}

#[test]
fn test_find_the_index_of_the_first_occurrence_in_a_string() {
    assert_eq!(
        find_the_index_of_the_first_occurrence_in_a_string(
            "sadbutsad".to_string(),
            "sad".to_string()
        ),
        0
    );
    assert_eq!(
        find_the_index_of_the_first_occurrence_in_a_string(
            "leetcode".to_string(),
            "leeto".to_string()
        ),
        -1
    );
}

#[test]
fn test_search_insert_position() {
    assert_eq!(search_insert_position(vec![1, 3, 5, 6], 5), 2);
    assert_eq!(search_insert_position(vec![1, 3, 5, 6], 2), 1);
    assert_eq!(search_insert_position(vec![1, 3, 5, 6], 7), 4);
}

#[test]
fn test_length_of_last_word() {
    assert_eq!(length_of_last_word("Hello World".to_string()), 5);
    assert_eq!(
        length_of_last_word("   fly me   to   the moon  ".to_string()),
        4
    );
    assert_eq!(length_of_last_word("luffy is still joyboy".to_string()), 6);
}

#[test]
fn test_plus_one() {
    assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    assert_eq!(plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    assert_eq!(plus_one(vec![9]), vec![1, 0]);
}

#[test]
fn test_add_binary() {
    assert_eq!(
        add_binary("11".to_string(), "1".to_string()),
        "100".to_string()
    );
    assert_eq!(
        add_binary("1010".to_string(), "1011".to_string()),
        "10101".to_string()
    );
}

#[test]
fn test_sqrt_x() {
    assert_eq!(sqrt_x(4), 2);
    assert_eq!(sqrt_x(8), 2);
}

#[test]
fn test_climbing_stairs() {
    assert_eq!(climbing_stairs(2), 2);
    assert_eq!(climbing_stairs(3), 3);
}

#[test]
fn test_remove_duplicates_from_sorted_list() {
    assert_eq!(
        remove_duplicates_from_sorted_list(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 2, next: None }))
            }))
        }))),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None }))
        }))
    );

    assert_eq!(
        remove_duplicates_from_sorted_list(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode { val: 3, next: None }))
                    }))
                }))
            }))
        }))),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None }))
            }))
        }))
    );
}

#[test]
fn test_merge_sorted_array() {
    let nums1 = &mut vec![1, 2, 3, 0, 0, 0];
    let nums2 = &mut vec![2, 5, 6];
    merge_sorted_array(nums1, 3, nums2, 3);
    assert_eq!(nums1[0], 1);
    assert_eq!(nums1[1], 2);
    assert_eq!(nums1[2], 2);
    assert_eq!(nums1[3], 3);
    assert_eq!(nums1[4], 5);
    assert_eq!(nums1[5], 6);

    let nums1 = &mut vec![1];
    let nums2 = &mut vec![];
    merge_sorted_array(nums1, 1, nums2, 0);
    assert_eq!(nums1[0], 1);

    let nums1 = &mut vec![0];
    let nums2 = &mut vec![1];
    merge_sorted_array(nums1, 0, nums2, 1);
    assert_eq!(nums1[0], 1);
}

#[test]
fn test_binary_tree_inorder_traversal() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: None,
        }))),
    })));
    assert_eq!(binary_tree_inorder_traversal(root), vec![1, 3, 2]);

    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        }))),
    })));
    assert_eq!(
        binary_tree_inorder_traversal(root),
        vec![4, 2, 6, 5, 7, 1, 3, 9, 8]
    );

    let root = None;
    assert_eq!(binary_tree_inorder_traversal(root), vec![]);

    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    })));
    assert_eq!(binary_tree_inorder_traversal(root), vec![1]);
}

#[test]
fn test_same_tree() {
    let p = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
    })));
    let q = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
    })));
    assert!(same_tree(p, q));

    let p = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
        right: None,
    })));
    let q = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
    })));
    assert!(!same_tree(p, q));

    let p = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }))),
    })));
    let q = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
    })));
    assert!(!same_tree(p, q));
}

#[test]
fn test_symmetric_tree() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        }))),
    })));
    assert!(symmetric_tree(root));

    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        }))),
    })));
    assert!(!symmetric_tree(root));
}

#[test]
fn test_maximum_depth_of_binary_tree() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    })));
    assert_eq!(maximum_depth_of_binary_tree(root), 3);

    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
    })));
    assert_eq!(maximum_depth_of_binary_tree(root), 2);
}

#[test]
fn test_convert_sorted_array_to_binary_search_tree() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 0,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: -3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: -10,
                left: None,
                right: None,
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            }))),
            right: None,
        }))),
    })));
    assert_eq!(
        convert_sorted_array_to_binary_search_tree(vec![-10, -3, 0, 5, 9]),
        root
    );

    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }))),
        right: None,
    })));
    assert_eq!(convert_sorted_array_to_binary_search_tree(vec![1, 3]), root);
}

#[test]
fn test_balanced_binary_tree() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    })));
    assert!(balanced_binary_tree(root));

    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
    })));
    assert!(!balanced_binary_tree(root));

    let root = None;
    assert!(balanced_binary_tree(root));
}

#[test]
fn test_minimum_depth_of_binary_tree() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    })));
    assert_eq!(minimum_depth_of_binary_tree(root), 2);

    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 6,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
        }))),
    })));
    assert_eq!(minimum_depth_of_binary_tree(root), 5);
}

#[test]
fn test_path_sum() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 11,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 13,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
            }))),
        }))),
    })));
    assert!(path_sum(root, 22));

    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
    })));
    assert!(!path_sum(root, 5));

    let root = None;
    assert!(!path_sum(root, 0));
}
