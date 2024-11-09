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
