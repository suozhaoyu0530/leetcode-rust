use std::cmp;

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
/// 为了确定cut的位置，那么可以得出nums1-left<=nums2-right   nums2-left<=nums1-right
/// 如果nums1-left > nums2-right 则切点过大，-1继续验证
/// 如果nums2-left > nums1-right 则切点过小，+1继续验证
/// 针对边界，如果是大于右边界的话，则可以赋值为i32的最大值，如果是小于左边界，可以赋值为i32的最小值
/// 将两个数组长度最小的的一个数组进行切分，针对不同的情况寻找到第二个数组的切点，进行比较
/// 然后针对奇偶数进行判定中位值
fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (v1, v2) = if nums1.len() <= nums2.len() {
        (&nums1, &nums2)
    } else {
        (&nums2, &nums1)
    };

    let mut mid_num = 0.0;
    
    let mut jud = true;

    let v1_len = v1.len();
    let v2_len = v2.len();

    let mut v1_cut = if v1_len == 0 {
        0
    } else {
        (v1_len as isize - 1) / 2
    };

    let is_ou = (v1_len + v2_len) % 2 == 0;

    while jud {
        let v2_cut = if v1_len == 0 {
            (v2_len as isize - 1) / 2
        } else if is_ou {
            ((v1_len + v2_len) as isize - 4) / 2 - v1_cut
        } else {
            ((v1_len + v2_len) as isize - 2) / 2 - v1_cut
        };

        let (v1_left, v1_right) = if v1_len == 0 {
            (i32::min_value(), i32::max_value())
        } else if v1_cut == -1 {
            (i32::min_value(), v1[(v1_cut + 1) as usize])
        } else if v1_cut as usize + 1 >= v1_len {
            (v1[v1_cut as usize], i32::max_value())
        } else {
            (v1[v1_cut as usize], v1[v1_cut as usize + 1])
        };

        let (v2_left, v2_right) = if v2_cut == -1 {
            (i32::min_value(), v2[(v2_cut + 1) as usize])
        } else if v2_cut as usize + 1 >= v2_len {
            (v2[v2_cut as usize], i32::max_value())
        } else {
            (v2[v2_cut as usize], v2[v2_cut as usize + 1])
        };

        if v1_left > v2_right {
            v1_cut = v1_cut - 1;
        } else if v2_left > v1_right {
            v1_cut = v1_cut + 1;
        } else {
            let left_max = cmp::max(v1_left, v2_left);
            let right_min = cmp::min(v1_right, v2_right);

            if is_ou {
                mid_num = (left_max + right_min) as f64 / 2.0;
            } else {
                mid_num = left_max as f64;
            }
            jud = false;
        }
    }

    mid_num
}

#[test]
fn example1() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];

    let mid_expected = 2_f64;

    assert_eq!(mid_expected, find_median_sorted_arrays(nums1, nums2));
}

#[test]
fn example2() {
    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4];

    let mid_expected = 2.5;

    assert_eq!(mid_expected, find_median_sorted_arrays(nums1, nums2));
}

#[test]
fn example3() {
    let nums1 = vec![1, 2, 4, 5];
    let nums2 = vec![3];

    let mid_expected = 3.0;

    assert_eq!(mid_expected, find_median_sorted_arrays(nums1, nums2));
}

#[test]
fn example4() {
    let nums1 = vec![2];
    let nums2 = vec![1, 2, 3];

    let mid_expected = 2.0;

    assert_eq!(mid_expected, find_median_sorted_arrays(nums1, nums2));
}

#[test]
fn example5() {
    let nums1 = vec![1];
    let nums2 = vec![1];

    let mid_expected = 1.0;

    assert_eq!(mid_expected, find_median_sorted_arrays(nums1, nums2));
}

#[test]
fn example6() {
    let nums1 = vec![1];
    let nums2 = vec![2, 3, 4];

    let mid_expected = 2.5;

    assert_eq!(mid_expected, find_median_sorted_arrays(nums1, nums2));
}

#[test]
fn example7() {
    let nums1 = vec![2];
    let nums2 = vec![1, 3, 4];

    let mid_expected = 2.5;

    assert_eq!(mid_expected, find_median_sorted_arrays(nums1, nums2));
}

#[test]
fn example8() {
    let nums1 = vec![];
    let nums2 = vec![2, 3, 4];

    let mid_expected = 3.0;

    assert_eq!(mid_expected, find_median_sorted_arrays(nums1, nums2));
}