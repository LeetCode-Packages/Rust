use leetcode_rs::easy;

#[test]
fn test_two_sum() {
    assert_eq!(easy::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(easy::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(easy::two_sum(vec![3, 3], 6), vec![0, 1]);
}
