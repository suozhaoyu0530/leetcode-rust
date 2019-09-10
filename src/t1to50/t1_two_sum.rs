use std::collections::HashMap;

/// 给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。
///你可以假设每种输入只会对应一个答案。但是，你不能重复利用这个数组中同样的元素。
///
///example:
///
///给定 nums = [2, 7, 11, 15], target = 9
///因为 nums[0] + nums[1] = 2 + 7 = 9
///所以返回 [0, 1]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut m = HashMap::new();

    for i in 0..nums.len() {
        let c = target-nums[i];

        if m.contains_key(&c) {
            return vec![*m.get(&c).unwrap(), i as i32];
        }

        m.insert(nums[i], i as i32);
    }

    panic!("have no result");
}

#[test]
fn test() {
    let v = vec![2, 7, 11, 15];
    let t = 9;

    let r_e = vec![0, 1];
    let r = two_sum(v, t);

    assert_eq!(r, r_e);
}