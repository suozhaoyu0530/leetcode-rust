/// 给定两个大小为 m 和 n 的有序数组 nums1 和 nums2。
/// 请你找出这两个有序数组的中位数，并且要求算法的时间复杂度为 O(log(m + n))。
/// 你可以假设 nums1 和 nums2 不会同时为空。
/// 
/// example 1:
/// nums1 = [1, 3]
/// nums2 = [2]
/// 则中位数是 2.0
///  
/// example 2:
/// nums1 = [1, 2]
/// nums2 = [3, 4]
/// 则中位数是 (2 + 3)/2 = 2.5
///
///
/// 思路：
/// nums1-left   nums1-right
/// nums2-left   nums2-right
/// max(left) <= min(right)
/// 然后针对奇偶数进行判定中位值
fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (v1, v2) = if nums1.len() < nums2.len() {
        (&nums1, &nums2)
    } else {
        (&nums2, &nums1)
    };

    let v1_len = v1.len();
    let v2_len = v2.len();
    let is_ou = if (v1_len+v2_len)%2 == 0{
        true
    } else {
        false
    };

    let mut cut = if v1_len % 2 {
        (v1_len + 1) / 2
    } else {
        v1_len / 2
    };

    let mid_num = 0_f64;

    let mut left_value = 0;
    let mut right_value = 0;

    while left_value <= right_value {
        let (v1_left, v1_right) = if v1.len() == 0 {
            (0, 0)
        } else if v1.len() <= cut+1 {
            (v1[cut], 0)
        } else {
            (v1[cut], v1[cut+1])
        };

        let v2_cut = (v1_len+v2_len)/2;
    }

    mid_num
}